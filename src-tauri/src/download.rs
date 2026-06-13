use crate::manifest::{build_filename, DownloadManifest, PaperMeta};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::Instant;

/// Result type for a handler: Ok(Some(path)) = success, Ok(None) = soft failure, Err(msg) = hard failure.
pub type HandlerResult = Result<Option<PathBuf>, String>;

/// Signature of a publisher download handler (async, boxed).
pub type HandlerFn = Box<dyn Fn(DownloadContext) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> + Send + Sync>;

/// Helper: build a standard reqwest client for handlers.
pub fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .expect("Failed to build reqwest client")
}

/// JSON schema for a single publisher handler entry.
#[derive(Debug, Deserialize)]
struct HandlerConfig {
    #[serde(default)]
    bypass: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    origin: Option<String>,
    #[serde(default)]
    scrape: Option<String>,
}

/// Context passed to each publisher handler.
/// Owned so it can be moved into async tasks.
#[derive(Clone)]
pub struct DownloadContext {
    pub doi: String,
    pub publisher_url: String,
    pub download_dir: std::path::PathBuf,
    pub filename: String,
    /// Base URL of the CF bypass proxy (e.g. "http://127.0.0.1:8000").
    pub cf_base_url: String,
}

pub struct DownloadService {
    storage_dir: PathBuf,
    manifest: DownloadManifest,
    naming_pattern: String,
    delay_seconds: u64,
    /// Per-publisher last download time for rate limiting.
    rate_limit: HashMap<String, Instant>,
    handlers: HashMap<String, HandlerFn>,
    manifest_path: PathBuf,
    cf_base_url: String,
}

impl DownloadService {
    pub fn new(
        storage_dir: &Path,
        naming_pattern: &str,
        delay_seconds: u64,
        cf_base_url: &str,
    ) -> Self {
        let manifest_path = storage_dir.join("manifest.json");
        let manifest = DownloadManifest::load(&manifest_path);
        let mut service = DownloadService {
            storage_dir: storage_dir.to_path_buf(),
            manifest,
            naming_pattern: naming_pattern.to_string(),
            delay_seconds,
            rate_limit: HashMap::new(),
            handlers: HashMap::new(),
            manifest_path,
            cf_base_url: cf_base_url.to_string(),
        };

        // Register all publisher handlers from bundled JSON config
        for (hostname, handler) in load_and_build_handlers() {
            service.register_handler(hostname, handler);
        }
        service
    }

    /// Register a publisher handler for a given hostname.
    pub fn register_handler(&mut self, hostname: &str, handler: HandlerFn) {
        self.handlers.insert(hostname.to_string(), handler);
    }

    /// Resolve a DOI to the publisher's actual URL via doi.org redirect.
    async fn resolve_doi(&self, client: &reqwest::Client, doi: &str) -> Result<String, String> {
        let url = format!("https://doi.org/{doi}");
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("DOI resolution failed: {e}"))?;
        let final_url = resp.url().to_string();
        Ok(final_url)
    }

    /// Extract hostname from a URL.
    fn extract_host(url: &str) -> Option<String> {
        url.split("://")
            .nth(1)?
            .split('/')
            .next()?
            .split(':')
            .next()
            .map(|s| s.to_string())
    }

    /// Rate-limit per publisher hostname.
    fn check_rate_limit(&self, hostname: &str) -> Result<(), String> {
        if let Some(last) = self.rate_limit.get(hostname) {
            let elapsed = last.elapsed().as_secs();
            if elapsed < self.delay_seconds {
                return Err(format!(
                    "Rate limited: {hostname}, wait {}s",
                    self.delay_seconds - elapsed
                ));
            }
        }
        Ok(())
    }

    /// Main download flow:
    /// 1. Check manifest for existing file
    /// 2. Resolve DOI → publisher URL
    /// 3. Rate-limit by publisher
    /// 4. Build filename
    /// 5. Dispatch to registered handler
    pub async fn download_by_doi(
        &mut self,
        doi: &str,
        meta: &PaperMeta,
        client: &reqwest::Client,
    ) -> Result<Option<PathBuf>, String> {
        // Check if already downloaded
        if let Some(path) = self.manifest.get(doi, &self.storage_dir) {
            return Ok(Some(path));
        }

        // Resolve DOI
        let publisher_url = self.resolve_doi(client, doi).await?;
        let host = Self::extract_host(&publisher_url)
            .ok_or_else(|| format!("Cannot extract host from: {publisher_url}"))?;

        // Rate limit
        self.check_rate_limit(&host)?;

        // Build filename
        let filename = build_filename(&self.naming_pattern, doi, meta);

        // Build context
        let ctx = DownloadContext {
            doi: doi.to_string(),
            publisher_url: publisher_url.clone(),
            download_dir: self.storage_dir.clone(),
            filename: filename.clone(),
            cf_base_url: self.cf_base_url.clone(),
        };

        // Find handler
        let handler = self
            .handlers
            .get(&host)
            .ok_or_else(|| format!("No handler for publisher: {host}"))?;

        // Download
        let result = handler(ctx).await?;

        // Record
        self.rate_limit.insert(host, Instant::now());
        if result.is_some() {
            self.manifest.set(doi, &filename);
            self.manifest.save(&self.manifest_path);
        }

        Ok(result)
    }
}

// ── Config-driven handler loading ──

/// Load bundled default_download_handlers.json and build the handler registry.
fn load_and_build_handlers() -> Vec<(&'static str, HandlerFn)> {
    let json_str = include_str!("../resources/default_download_handlers.json");
    let map: HashMap<String, HandlerConfig> = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to parse download handlers config: {e}");
            return vec![];
        }
    };

    let mut entries: Vec<(String, HandlerConfig)> = map.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut handlers: Vec<(&'static str, HandlerFn)> = Vec::new();

    for (hostname, config) in entries {
        let hostname: &'static str = Box::leak(hostname.into_boxed_str());
        let handler = build_handler(config);
        handlers.push((hostname, handler));
    }

    handlers
}

fn build_handler(config: HandlerConfig) -> HandlerFn {
    let config: &'static HandlerConfig = Box::leak(Box::new(config));

    let origin_regex: Option<&'static Regex> = config
        .origin
        .as_ref()
        .and_then(|s| Regex::new(s).ok())
        .map(|re| Box::leak(Box::new(re)) as &'static Regex);

    let scrape_regex: Option<&'static Regex> = config
        .scrape
        .as_ref()
        .and_then(|s| Regex::new(s).ok())
        .map(|re| Box::leak(Box::new(re)) as &'static Regex);

    Box::new(move |ctx: DownloadContext| {
        Box::pin(async move {
            let final_url = match resolve_url(&ctx, config, origin_regex, scrape_regex).await {
                Some(u) => u,
                None => return Ok(None),
            };

            let client = build_client();
            if config.bypass.as_deref() == Some("cloudflare") {
                return Ok(crate::cf_proxy::download_via_cf(
                    &client,
                    &ctx.cf_base_url,
                    &final_url,
                    &ctx.download_dir,
                    &ctx.filename,
                )
                .await);
            }

            Ok(download_direct(&client, &final_url, &ctx.download_dir, &ctx.filename).await)
        }) as Pin<Box<dyn Future<Output = HandlerResult> + Send>>
    })
}

/// Resolve the final download URL from config placeholders.
/// Returns None if the URL template is missing or captures fail.
async fn resolve_url(
    ctx: &DownloadContext,
    config: &HandlerConfig,
    origin_regex: Option<&Regex>,
    scrape_regex: Option<&Regex>,
) -> Option<String> {
    let url_template = config.url.as_ref()?;

    // Resolve {origin[n]} captures from publisher_url
    let origin_caps: Vec<String> = origin_regex
        .and_then(|re| re.captures(&ctx.publisher_url))
        .map(|caps| {
            (0..caps.len())
                .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect()
        })
        .unwrap_or_default();

    // Resolve {scrape[n]} captures from page HTML
    let scrape_caps: Vec<String> = if scrape_regex.is_some() {
        let client = build_client();
        scrape_captures(&client, &ctx.publisher_url, scrape_regex).await
    } else {
        Vec::new()
    };

    let mut final_url = url_template.replace("{doi}", &ctx.doi);
    for (i, val) in origin_caps.iter().enumerate() {
        final_url = final_url.replace(&format!("{{origin[{i}]}}"), val);
    }
    for (i, val) in scrape_caps.iter().enumerate() {
        final_url = final_url.replace(&format!("{{scrape[{i}]}}"), val);
    }

    Some(final_url)
}

async fn scrape_captures(
    client: &reqwest::Client,
    publisher_url: &str,
    scrape_regex: Option<&Regex>,
) -> Vec<String> {
    let re = match scrape_regex {
        Some(r) => r,
        None => return Vec::new(),
    };
    let resp = match client.get(publisher_url).send().await {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    let html = match resp.text().await {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    let caps = match re.captures(&html) {
        Some(c) => c,
        None => return Vec::new(),
    };
    (0..caps.len())
        .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
        .collect()
}

async fn download_direct(
    client: &reqwest::Client,
    url: &str,
    download_dir: &Path,
    filename: &str,
) -> Option<PathBuf> {
    let dest = download_dir.join(filename);
    let resp = client.get(url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let bytes = resp.bytes().await.ok()?;
    if bytes.is_empty() {
        return None;
    }
    if let Some(parent) = dest.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&dest, &bytes).ok()?;
    Some(dest)
}
