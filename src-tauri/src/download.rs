use crate::manifest::{build_filename, DownloadManifest, PaperMeta};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Publisher-specific PDF download strategy.
/// Handlers are registered in stage 1e (handlers/ module).
pub type PublisherHandler = fn(
    client: &reqwest::Client,
    doi: &str,
    publisher_url: &str,
    download_dir: &Path,
    filename: &str,
) -> Result<Option<PathBuf>, String>;

pub struct DownloadService {
    storage_dir: PathBuf,
    manifest: DownloadManifest,
    naming_pattern: String,
    delay_seconds: u64,
    /// Per-publisher last download time for rate limiting.
    rate_limit: HashMap<String, Instant>,
    handlers: HashMap<String, PublisherHandler>,
    manifest_path: PathBuf,
}

impl DownloadService {
    pub fn new(storage_dir: &Path, naming_pattern: &str, delay_seconds: u64) -> Self {
        let manifest_path = storage_dir.join("manifest.json");
        let manifest = DownloadManifest::load(&manifest_path);
        DownloadService {
            storage_dir: storage_dir.to_path_buf(),
            manifest,
            naming_pattern: naming_pattern.to_string(),
            delay_seconds,
            rate_limit: HashMap::new(),
            handlers: HashMap::new(),
            manifest_path,
        }
    }

    /// Register a publisher handler for a given hostname.
    pub fn register_handler(&mut self, hostname: &str, handler: PublisherHandler) {
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
    /// 2. Rate-limit by publisher
    /// 3. Resolve DOI → publisher URL
    /// 4. Dispatch to registered handler
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

        // Find handler
        let handler = self
            .handlers
            .get(&host)
            .copied()
            .ok_or_else(|| format!("No handler for publisher: {host}"))?;

        // Download
        let result = handler(client, doi, &publisher_url, &self.storage_dir, &filename)?;

        // Record
        self.rate_limit.insert(host, Instant::now());
        if result.is_some() {
            self.manifest.set(doi, &filename);
            self.manifest.save(&self.manifest_path);
        }

        Ok(result)
    }
}
