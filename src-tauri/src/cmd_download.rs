/// Tauri commands for PDF download functionality.
use crate::cf_proxy;
use crate::config::get;
use crate::download::DownloadService;
use crate::manifest::PaperMeta;
use crate::paper::Paper;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tokio::sync::Mutex;

/// Global download service instance.
static DOWNLOAD_SERVICE: OnceCell<Mutex<DownloadService>> = OnceCell::new();

/// Lazy-init the download service from config values.
fn get_download_service() -> &'static Mutex<DownloadService> {
    DOWNLOAD_SERVICE.get_or_init(|| {
        let storage_dir = get("download_dir")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./papers"));
        let naming_pattern = "{doi}.pdf";
        let delay_seconds = 5u64;
        let cf_host = get("citation_cf_host")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .filter(|s| !s.is_empty())
            .unwrap_or_default();
        let cf_port: u16 = get("citation_cf_port")
            .and_then(|v| v.as_str().map(|s| s.parse().ok()).flatten())
            .filter(|p| *p > 0)
            .unwrap_or(8000);
        let cf_base_url = cf_proxy::normalize_base_url(&cf_host, cf_port);

        let service = DownloadService::new(
            &storage_dir,
            naming_pattern,
            delay_seconds,
            &cf_base_url,
        );
        Mutex::new(service)
    })
}

/// Test if the CloudflareBypass service is reachable.
#[tauri::command]
pub async fn test_cf_bypass(host: String, port: u16) -> Result<bool, String> {
    let base_url = cf_proxy::normalize_base_url(&host, port);
    Ok(cf_proxy::check_cf_bypass(&base_url).await)
}

/// Download a paper PDF by DOI.
/// Returns the downloaded file path on success, or an error message.
#[tauri::command]
pub async fn download_citation_pdf(doi: String, paper: Paper) -> Result<String, String> {
    let service = get_download_service();
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

    let meta = PaperMeta {
        title: paper.title.clone().unwrap_or_default(),
        authors: paper.authors.clone(),
        year: paper.year,
        doi: Some(doi.clone()),
    };

    let mut svc = service.lock().await;
    let result = svc
        .download_by_doi(&doi, &meta, &client)
        .await
        .map_err(|e| format!("Download failed: {e}"))?;

    match result {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("Download failed: no handler available or CF bypass unreachable".to_string()),
    }
}
