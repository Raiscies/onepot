use crate::download::{DownloadClient, DownloadContext, HandlerResult};
use once_cell::sync::Lazy;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;

/// Precompiled regex: extract PII from linkinghub URL path (e.g. `pii/S1234567890X...`).
static PII_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"pii/([^/?]+)").unwrap());

/// Marker that starts the ScienceDirect preloaded state JSON block.
static PRELOAD_MARKER: &str = "__PRELOADED_STATE__";

/// End of a `<script>` tag — bounds the preloaded state block.
static SCRIPT_END: &str = "</script>";

/// Precompiled regex: extract `md5` value (32 hex chars) from preloaded JSON.
static MD5_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""md5"\s*:\s*"([a-f0-9]{32})""#).unwrap());

/// Precompiled regex: extract `pid` value (PDF filename) from preloaded JSON.
static PID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""pid"\s*:\s*"([^"]+)""#).unwrap());

/// Bridging function: matches the `CustomHandlerFn` signature.
pub(super) fn handler<'a>(
    client: &'a DownloadClient,
    ctx: &'a DownloadContext,
) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + 'a>> {
    Box::pin(handle(client, ctx))
}

/// Elsevier linkinghub → ScienceDirect: extract PII, parse `__PRELOADED_STATE__` JSON
/// for md5/pid, then download PDF directly.
async fn handle(client: &DownloadClient, ctx: &DownloadContext) -> HandlerResult {
    let pii = PII_RE
        .captures(&ctx.publisher_url)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| "failed to extract PII from linkinghub URL".to_string())?;

    let sd_url = format!("https://www.sciencedirect.com/science/article/pii/{pii}");
    log::info!("elsevier PII={pii} → {sd_url}");

    let html = client
        .fetch_page(&sd_url)
        .await
        .ok_or_else(|| "failed to fetch sciencedirect page".to_string())?;

    // Locate the __PRELOADED_STATE__ JSON block, bounded by </script>.
    let start = html
        .find(PRELOAD_MARKER)
        .ok_or_else(|| "page does not contain __PRELOADED_STATE__".to_string())?;

    let block = &html[start..];
    let script_end = block
        .find(SCRIPT_END)
        .ok_or_else(|| "unterminated script block after __PRELOADED_STATE__".to_string())?;
    let block = &block[..script_end];

    // Extract md5 (32 hex chars) from the JSON block.
    let md5 = MD5_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| "md5 not found in __PRELOADED_STATE__".to_string())?;

    // Extract pid (e.g. "1-s2.0-S1569190X19301005-main.pdf") from the JSON block.
    let pid = PID_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| "pid not found in __PRELOADED_STATE__".to_string())?;

    let pdf_url = format!("https://www.sciencedirect.com/science/article/pii/{pii}/pdfft?md5={md5}&pid={pid}");
    log::info!("elsevier PDF URL: {pdf_url}");

    client
        .download(&pdf_url, &ctx.download_dir, &ctx.filename, &ctx.doi, &[("Referer", &sd_url)])
        .await
        .map(Some)
        .ok_or_else(|| "elsevier PDF download failed".to_string())
}
