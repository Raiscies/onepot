/// CloudflareBypass proxy client.
/// Communicates with the external CF bypass Python service via HTTP.
use serde::Deserialize;
use tauri::Manager;

/// Health check response from /cache/stats
#[derive(Debug, Deserialize)]
pub struct CacheStats {
    // The exact fields vary; we only care that the endpoint responds with valid JSON.
}

/// Normalize a CF host config value into a base URL.
/// Accepts: "127.0.0.1", "127.0.0.1:8000", "http://127.0.0.1:8000", "https://proxy.example.com"
/// Falls back to "http://127.0.0.1:8000" if empty.
pub fn normalize_base_url(host: &str, port: u16) -> String {
    let host = host.trim();
    if host.is_empty() {
        return format!("http://127.0.0.1:{port}");
    }
    if host.starts_with("http://") || host.starts_with("https://") {
        return host.to_string();
    }
    // Check if port is already included in the host string
    if host.contains(':') {
        return format!("http://{host}");
    }
    format!("http://{host}:{port}")
}

/// Check if the CF bypass service is reachable at the given base URL.
pub async fn check_cf_bypass(base_url: &str) -> bool {
    let url = format!("{base_url}/cache/stats");
    match reqwest::get(&url).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// Download a file through the CF bypass proxy using the mirror endpoint.
/// The proxy forwards the request to `target_url` with CF bypass cookies.
///
/// Returns `Some(path)` on success, `None` on failure.
pub async fn download_via_cf(
    client: &reqwest::Client,
    base_url: &str,
    target_url: &str,
    doi: &str,
    download_dir: &std::path::Path,
    filename: &str,
) -> Option<std::path::PathBuf> {
    // Decompose target_url into host + path for the x-hostname mirror protocol.
    let (target_host, path) = split_url(target_url);

    let proxy_url = format!("{base_url}{path}");
    let dest = download_dir.join(filename);

    // Emit indefinite spinner signal (total=0 means indeterminate)
    emit_progress(doi, 0, 0);

    let resp = match client
        .get(&proxy_url)
        .header("x-hostname", &target_host)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            log::warn!("CF bypass request failed: {e}");
            return None;
        }
    };

    if !resp.status().is_success() {
        log::warn!("CF bypass returned status {}", resp.status());
        return None;
    }

    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            log::warn!("CF bypass read body failed: {e}");
            return None;
        }
    };

    if bytes.is_empty() {
        log::warn!("CF bypass returned empty body");
        return None;
    }

    let len = bytes.len() as u64;
    if let Some(parent) = dest.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::write(&dest, &bytes) {
        Ok(_) => {
            emit_progress(doi, len, len);
            Some(dest)
        }
        Err(e) => {
            log::warn!("Failed to write downloaded file: {e}");
            None
        }
    }
}

fn emit_progress(doi: &str, downloaded: u64, total: u64) {
    if let Some(handle) = crate::APP.get() {
        let _ = handle.emit_all(
            "download_progress",
            serde_json::json!({ "doi": doi, "downloaded": downloaded, "total": total }),
        );
    }
}

/// Split a full URL into (host, path).
/// e.g. "https://dl.acm.org/doi/pdf/10.1145/123" → ("dl.acm.org", "/doi/pdf/10.1145/123")
fn split_url(url: &str) -> (String, String) {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let slash_pos = without_scheme.find('/');
    match slash_pos {
        Some(pos) => {
            let host = without_scheme[..pos].to_string();
            let path = without_scheme[pos..].to_string();
            (host, path)
        }
        None => (without_scheme.to_string(), "/".to_string()),
    }
}
