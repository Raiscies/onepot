use crate::manifest::{build_filename, DownloadManifest, PaperMeta};
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tauri::Manager;

/// Default headers applied to every outgoing request.
static DEFAULT_HEADERS: Lazy<Vec<(&str, &str)>> = Lazy::new(|| {
    vec![
        ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
        ("Accept", "*/*"),
    ]
});

/// Result type for a handler: Ok(Some(path)) = success, Ok(None) = no handler, Err(msg) = failure.
pub type HandlerResult = Result<Option<PathBuf>, String>;

/// Structured download outcome for the frontend.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DownloadOutcome {
    Success { path: String },
    NoHandler { host: String },
    Failed { reason: String },
}

impl DownloadOutcome {
    pub fn status(&self) -> &'static str {
        match self {
            DownloadOutcome::Success { .. } => "success",
            DownloadOutcome::NoHandler { .. } => "no_handler",
            DownloadOutcome::Failed { .. } => "failed",
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self {
            DownloadOutcome::Success { path } => Some(path),
            _ => None,
        }
    }
}

/// Global handler table, initialised lazily from bundled JSON.
static HANDLER_TABLE: OnceCell<HashMap<&'static str, HandlerEntry>> = OnceCell::new();

/// One entry per publisher hostname.
struct HandlerEntry {
    bypass: Option<String>,
    url_template: String,
    origin_regex: Option<Regex>,
    scraped: Option<ScrapedValue>,
}

/// Transparent HTTP client — handlers never see bypass/proxy details.
pub struct DownloadClient {
    http: reqwest::Client,
    strategy: BypassStrategy,
    cf_base_url: String,
    proxy_url: Option<String>,
    cf_use_proxy: bool,
}

enum BypassStrategy {
    Direct,
    CloudflareBypass,
}

/// Simplified context — no proxy / CF fields.
#[derive(Clone)]
pub struct DownloadContext {
    pub doi: String,
    pub publisher_url: String,
    pub download_dir: PathBuf,
    pub filename: String,
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
    scrape: Option<ScrapeConfig>,
}

/// Scrape configuration: CSS selector with @/@@ extraction + optional regex filter.
#[derive(Debug, Deserialize)]
struct ScrapeConfig {
    /// CSS selector, optionally suffixed with `@attr` (attribute) or `@@` (text content).
    #[serde(default)]
    select: Option<String>,
    /// Optional regex to filter/transform the extracted value. Capture groups become {scrape[0]}..{scrape[N]}.
    #[serde(default)]
    target: Option<String>,
}

/// Parsed form of `select`: the CSS selector and what to extract from the matched element.
#[derive(Debug, Clone)]
enum ExtractMode {
    /// Return the element's outer HTML (the default when no @ suffix).
    Html,
    /// Return the value of the named attribute.
    Attr(String),
    /// Return the element's text content (all text nodes, no tags).
    Text,
}

/// Resolved scrape config, ready to use at runtime.
#[derive(Debug, Clone)]
struct ScrapedValue {
    extract: ExtractMode,
    /// The raw CSS selector (without @/@suffix) to pass to scraper::Selector::parse.
    selector: Option<String>,
    /// Optional regex applied to the extracted value.
    regex: Option<regex::Regex>,
}

pub struct DownloadService {
    storage_dir: PathBuf,
    manifest: DownloadManifest,
    naming_pattern: String,
    delay_seconds: u64,
    rate_limit: HashMap<String, Instant>,
    manifest_path: PathBuf,
    cf_base_url: String,
    proxy_url: Option<String>,
    cf_use_proxy: bool,
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
        DownloadService {
            storage_dir: storage_dir.to_path_buf(),
            manifest,
            naming_pattern: naming_pattern.to_string(),
            delay_seconds,
            rate_limit: HashMap::new(),
            manifest_path,
            cf_base_url: cf_base_url.to_string(),
            proxy_url: None,
            cf_use_proxy: false,
        }
    }

    /// Update the CF bypass base URL at runtime without restart.
    pub fn update_cf_base_url(&mut self, host: &str, port: u16) {
        self.cf_base_url = crate::cf_proxy::normalize_base_url(host, port);
        log::info!("CF bypass URL updated to {}", self.cf_base_url);
    }

    /// Update the HTTP proxy for non-CF downloads at runtime.
    pub fn update_proxy(&mut self, host: &str, port: u16) {
        if host.is_empty() || port == 0 {
            self.proxy_url = None;
            log::info!("Download proxy disabled");
        } else {
            self.proxy_url = Some(format!("http://{host}:{port}"));
            log::info!("Download proxy updated to {}", self.proxy_url.as_ref().unwrap());
        }
    }

    /// Set whether CF bypass also routes through the HTTP proxy.
    pub fn update_cf_use_proxy(&mut self, enable: bool) {
        self.cf_use_proxy = enable;
        log::info!("CF bypass use proxy: {enable}");
    }

    /// Check if a PDF for the given DOI already exists in cache.
    /// Cleans stale manifest entry if the file was deleted.
    pub fn check_existing(&mut self, doi: &str) -> Option<PathBuf> {
        let path = self.manifest.get_and_clean(doi, &self.storage_dir);
        if path.is_none() {
            self.manifest.save(&self.manifest_path);
        }
        path
    }

    /// Resolve a DOI to the publisher's actual URL via doi.org redirect.
    async fn resolve_doi(&self, doi: &str) -> Result<String, String> {
        let client = reqwest::Client::builder()
            .no_proxy()
            // .http1_only()
            .timeout(std::time::Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {e}"))?;
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

    pub async fn download_by_doi(&mut self, meta: &PaperMeta) -> DownloadOutcome {
        let doi = match &meta.doi {
            Some(d) => d.as_str(),
            None => return DownloadOutcome::Failed { reason: "PaperMeta missing DOI".to_string() },
        };

        if let Some(path) = self.manifest.get_and_clean(doi, &self.storage_dir) {
            self.manifest.save(&self.manifest_path);
            return DownloadOutcome::Success { path: path.to_string_lossy().to_string() };
        }

        let publisher_url = match self.resolve_doi(doi).await {
            Ok(u) => u,
            Err(e) => return DownloadOutcome::Failed { reason: e },
        };
        let host = match Self::extract_host(&publisher_url) {
            Some(h) => h,
            None => return DownloadOutcome::Failed {
                reason: format!("Cannot extract host from: {publisher_url}"),
            },
        };

        if let Err(e) = self.check_rate_limit(&host) {
            return DownloadOutcome::Failed { reason: e };
        }

        let entry = match handler_table().get(host.as_str()) {
            Some(e) => e,
            None => return DownloadOutcome::NoHandler { host },
        };

        let client = DownloadClient::build(entry.bypass.as_deref(), &self.cf_base_url)
            .with_proxy(self.proxy_url.as_deref())
            .with_cf_use_proxy(self.cf_use_proxy);

        let filename = build_filename(&self.naming_pattern, meta);
        let ctx = DownloadContext {
            doi: doi.to_string(),
            publisher_url: publisher_url.clone(),
            download_dir: self.storage_dir.clone(),
            filename: filename.clone(),
        };

        match default_handler(&client, &ctx, entry).await {
            Ok(Some(path)) => {
                self.rate_limit.insert(host, Instant::now());
                self.manifest.set(doi, &filename);
                self.manifest.save(&self.manifest_path);
                DownloadOutcome::Success { path: path.to_string_lossy().to_string() }
            }
            Ok(None) => DownloadOutcome::Failed { reason: "handler returned empty result".to_string() },
            Err(e) => DownloadOutcome::Failed { reason: e },
        }
    }
}

// ── Handler table initialisation ──

fn handler_table() -> &'static HashMap<&'static str, HandlerEntry> {
    HANDLER_TABLE.get_or_init(|| {
        let json_str = include_str!("../resources/default_download_handlers.json");
        let map: HashMap<String, HandlerConfig> = serde_json::from_str(json_str)
            .expect("Failed to parse default_download_handlers.json");

        let mut table = HashMap::new();
        for (hostname, config) in map {
            let hostname: &'static str = Box::leak(hostname.into_boxed_str());
            let scraped = config.scrape.map(|sc| {
                let (sel, extract) = parse_scrape_select(sc.select.as_deref());
                ScrapedValue {
                    extract,
                    selector: sel.map(|s| s.to_string()),
                    regex: sc.target.as_ref().and_then(|s| Regex::new(s).ok()),
                }
            });
            table.insert(
                hostname,
                HandlerEntry {
                    bypass: config.bypass,
                    url_template: config.url.unwrap_or_default(),
                    origin_regex: config.origin.and_then(|s| Regex::new(&s).ok()),
                    scraped,
                },
            );
        }
        table
    })
}

// ── DownloadClient ──

impl DownloadClient {
    /// Build a client for a specific bypass strategy.
    pub fn build(bypass: Option<&str>, base_url: &str) -> Self {
        let strategy = match bypass {
            Some("cloudflare") => BypassStrategy::CloudflareBypass,
            _ => BypassStrategy::Direct,
        };
        let http = reqwest::Client::builder()
            .no_proxy()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .expect("Failed to build reqwest client");

        DownloadClient {
            http,
            strategy,
            cf_base_url: base_url.to_string(),
            proxy_url: None,
            cf_use_proxy: false,
        }
    }

    // /// Resolve a DOI to the publisher's actual URL via doi.org redirect.
    // pub async fn resolve_doi(&self, doi: &str) -> Result<String, String> {
    //     let url = format!("https://doi.org/{doi}");
    //     let resp = self.http
    //         .get(&url)
    //         .send()
    //         .await
    //         .map_err(|e| format!("DOI resolution failed: {e}"))?;
    //     Ok(resp.url().to_string())
    // }

    /// Set the optional HTTP proxy (called before each use).
    pub fn with_proxy(mut self, proxy_url: Option<&str>) -> Self {
        self.proxy_url = proxy_url.map(|s| s.to_string());
        self
    }

    /// Set whether CF bypass also routes through the HTTP proxy.
    pub fn with_cf_use_proxy(mut self, cf_use_proxy: bool) -> Self {
        self.cf_use_proxy = cf_use_proxy;
        self
    }

    /// Fetch a page — transparently routes through CF bypass if needed.
    pub async fn fetch_page(&self, url: &str) -> Option<String> {
        match self.strategy {
            BypassStrategy::CloudflareBypass => {
                let proxy = if self.cf_use_proxy { self.proxy_url.as_deref() } else { None };
                crate::cf_proxy::fetch_via_cf(&self.http, &self.cf_base_url, url, proxy).await
            }
            BypassStrategy::Direct => {
                let client = match &self.proxy_url {
                    Some(p) => reqwest::Client::builder()
                        .no_proxy()
                        .cookie_store(true)
                        .proxy(reqwest::Proxy::all(p).ok()?)
                        .timeout(std::time::Duration::from_secs(120))
                        .redirect(reqwest::redirect::Policy::limited(10))
                        .build()
                        .ok()?,
                    None => self.http.clone(),
                };
                let mut req = client.get(url);
                for (k, v) in DEFAULT_HEADERS.iter() {
                    req = req.header(*k, *v);
                }
                let resp = req.send().await.ok()?;
                resp.text().await.ok()
            }
        }
    }

    /// Download a file — transparently routes through CF bypass / proxy.
    pub async fn download(
        &self,
        url: &str,
        download_dir: &Path,
        filename: &str,
        doi: &str,
        extra_headers: &[(&str, &str)],
    ) -> Option<PathBuf> {
        match self.strategy {
            BypassStrategy::CloudflareBypass => {
                log::debug!("download with strategy: CloudflareBypass");
                let proxy = if self.cf_use_proxy { self.proxy_url.as_deref() } else { None };
                crate::cf_proxy::download_via_cf(
                    &self.http, &self.cf_base_url, url, proxy, doi, download_dir, filename,
                )
                .await
            }
            BypassStrategy::Direct => {
                log::debug!("download with strategy: Direct");
                self.download_direct(url, download_dir, filename, doi, extra_headers).await
            }
        }
    }

    /// Direct download with optional proxy.
    async fn download_direct(
        &self,
        url: &str,
        download_dir: &Path,
        filename: &str,
        doi: &str,
        extra_headers: &[(&str, &str)],
    ) -> Option<PathBuf> {
        let client = match &self.proxy_url {
            Some(p) => reqwest::Client::builder()
                .no_proxy()
                .cookie_store(true)
                .proxy(reqwest::Proxy::all(p).ok()?)
                .timeout(std::time::Duration::from_secs(120))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .ok()?,
            None => self.http.clone(),
        };
        let dest = download_dir.join(filename);
        let mut req = client.get(url);
        for (k, v) in DEFAULT_HEADERS.iter().chain(extra_headers.iter()) {
            req = req.header(*k, *v);
        }
        let resp = req.send().await.ok()?;
        if !resp.status().is_success() {
            log::warn!("download failed: {} {} ({} bytes)", resp.status(), url, resp.content_length().unwrap_or(0));
            return None;
        }

        let total = resp.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;
        let mut buf = Vec::new();

        use futures::StreamExt;
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.ok()?;
            downloaded += chunk.len() as u64;
            buf.extend_from_slice(&chunk);
            if total > 0 {
                emit_progress(doi, downloaded, total);
            }
        }

        if buf.is_empty() {
            return None;
        }
        emit_progress(doi, total, total);

        if let Some(parent) = dest.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&dest, &buf).ok()?;
        Some(dest)
    }
}

// ── default_handler ──

async fn default_handler(
    client: &DownloadClient,
    ctx: &DownloadContext,
    entry: &HandlerEntry,
) -> HandlerResult {
    let final_url = resolve_download_url(client, ctx, entry).await
        .ok_or_else(|| "failed to resolve download URL".to_string())?;

    log::info!("Download resolved URL for doi={}: {}", ctx.doi, final_url);

    client
        .download(
            &final_url, 
            &ctx.download_dir, 
            &ctx.filename, 
            &ctx.doi, 
            // &[("Referer", &ctx.publisher_url)]
            &[]
        )
        .await
        .map(Some)
        .ok_or_else(|| "download failed".to_string())
}

// ── URL resolution ──

async fn resolve_download_url(
    client: &DownloadClient,
    ctx: &DownloadContext,
    entry: &HandlerEntry,
) -> Option<String> {
    let url_template = &entry.url_template;
    if url_template.is_empty() {
        return None;
    }

    let origin_caps: Vec<String> = entry
        .origin_regex
        .as_ref()
        .and_then(|re| re.captures(&ctx.publisher_url))
        .map(|caps| {
            (0..caps.len())
                .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect()
        })
        .unwrap_or_default();

    let scrape_caps: Vec<String> = match &entry.scraped {
        Some(sv) => scrape_captures(client, &ctx.publisher_url, sv).await.unwrap_or_default(),
        None => Vec::new(),
    };

    let mut result = url_template.replace("{doi}", &ctx.doi);
    for (i, val) in origin_caps.iter().enumerate() {
        result = result.replace(&format!("{{origin[{i}]}}"), val);
    }
    for (i, val) in scrape_caps.iter().enumerate() {
        result = result.replace(&format!("{{scrape[{i}]}}"), val);
    }
    Some(result)
}

async fn scrape_captures(
    client: &DownloadClient,
    publisher_url: &str,
    scraped: &ScrapedValue,
) -> Option<Vec<String>> {
    let html = client.fetch_page(publisher_url).await?;

    let raw_value = match &scraped.selector {
        Some(sel) => extract_from_element(&html, sel, &scraped.extract)?,
        None => html,
    };

    match &scraped.regex {
        Some(re) => {
            let caps = re.captures(&raw_value)?;
            Some(
                (0..caps.len())
                    .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
                    .collect(),
            )
        }
        None => Some(vec![raw_value]),
    }
}

/// Use `scraper` crate to find the first element matching `selector`,
/// then extract content according to `mode`.
fn extract_from_element(html: &str, selector: &str, mode: &ExtractMode) -> Option<String> {
    let document = scraper::Html::parse_document(html);
    let sel = match scraper::Selector::parse(selector) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("scrape: invalid selector \"{selector}\": {e:?}");
            return None;
        }
    };
    let element = match document.select(&sel).next() {
        Some(el) => el,
        None => {
            log::warn!("scrape: selector \"{selector}\" matched no elements");
            return None;
        }
    };

    let element_html = element.html();
    let result = match mode {
        ExtractMode::Html => Some(element_html.clone()),
        ExtractMode::Attr(name) => element.attr(name).map(|v| v.to_string()),
        ExtractMode::Text => {
            let text: String = element.text().collect();
            Some(text)
        }
    };

    log::info!(
        "scrape select=\"{selector}\" || element=\"{element_html}\" | extracted=\"{}\"",
        result.as_deref().unwrap_or("<None>")
    );

    result
}

/// Parse the raw `select` field into (css_selector, ExtractMode).
fn parse_scrape_select(raw: Option<&str>) -> (Option<&str>, ExtractMode) {
    let raw = match raw {
        Some(r) => r,
        None => return (None, ExtractMode::Html),
    };

    if raw.ends_with("@@") {
        let sel = &raw[..raw.len() - 2];
        return (Some(sel), ExtractMode::Text);
    }

    if let Some(pos) = raw.rfind('@') {
        let sel = &raw[..pos];
        let attr = &raw[pos + 1..];
        return (Some(sel), ExtractMode::Attr(attr.to_string()));
    }

    (Some(raw), ExtractMode::Html)
}

fn emit_progress(doi: &str, downloaded: u64, total: u64) {
    if let Some(handle) = crate::APP.get() {
        let _ = handle.emit_all(
            "download_progress",
            serde_json::json!({ "doi": doi, "downloaded": downloaded, "total": total }),
        );
    }
}
