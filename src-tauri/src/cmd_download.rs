/// Tauri commands for PDF download functionality.
use crate::cf_proxy;
use crate::config::get;
use crate::download::DownloadService;
use crate::manifest::PaperMeta;
use crate::paper::Paper;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tauri::Manager;
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

/// Update CF bypass config at runtime without restart.
#[tauri::command]
pub async fn update_cf_config(host: String, port: u16, use_proxy: bool) -> Result<(), String> {
    let service = get_download_service();
    let mut svc = service.lock().await;
    svc.update_cf_base_url(&host, port);
    svc.update_cf_use_proxy(use_proxy);
    Ok(())
}

/// Update download HTTP proxy at runtime.
#[tauri::command]
pub async fn update_proxy_config(host: String, port: u16) -> Result<(), String> {
    let service = get_download_service();
    let mut svc = service.lock().await;
    svc.update_proxy(&host, port);
    Ok(())
}

/// Test if the CloudflareBypass service is reachable.
#[tauri::command]
pub async fn test_cf_bypass(host: String, port: u16) -> Result<bool, String> {
    let base_url = cf_proxy::normalize_base_url(&host, port);
    Ok(cf_proxy::check_cf_bypass(&base_url).await)
}

/// Check if a PDF for the given DOI already exists in the download cache.
/// Returns the file path if found, or null.
#[tauri::command]
pub async fn check_pdf_exists(doi: String) -> Result<Option<String>, String> {
    let service = get_download_service();
    let mut svc = service.lock().await;
    let path = svc.check_existing(&doi);
    match &path {
        Some(p) => log::info!("check_pdf_exists doi={doi}: found at {}", p.display()),
        None => log::info!("check_pdf_exists doi={doi}: not found"),
    }
    Ok(path.map(|p| p.to_string_lossy().to_string()))
}

/// Download a paper PDF by DOI.
/// Returns a JSON DownloadOutcome: {status: "success"|"no_handler"|"failed", ...}
#[tauri::command]
pub async fn download_citation_pdf(doi: String, paper: Paper) -> Result<String, String> {
    let service = get_download_service();
    let meta = PaperMeta {
        title: paper.title.clone().unwrap_or_default(),
        authors: paper.authors.clone(),
        year: paper.year,
        doi: Some(doi.clone()),
    };

    let mut svc = service.lock().await;
    let outcome = svc.download_by_doi(&meta).await;

    // Emit download_finished event for frontend state sync
    if let Some(handle) = crate::APP.get() {
        let _ = handle.emit_all(
            "download_finished",
            serde_json::json!({ "doi": doi, "status": outcome.status(), "path": outcome.path() }),
        );
    }

    serde_json::to_string(&outcome).map_err(|e| format!("Serialization failed: {e}"))
}
