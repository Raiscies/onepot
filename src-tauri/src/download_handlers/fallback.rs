use crate::download::{DownloadClient, DownloadContext, HandlerResult};
use std::future::Future;
use std::pin::Pin;

/// Fallback handler invoked when no publisher-specific handler matches.
/// Checks whether the publisher URL points directly to a PDF and downloads it,
/// otherwise returns a failure.
pub(crate) fn fallback_handler<'a>(
    client: &'a DownloadClient,
    ctx: &'a DownloadContext,
) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + 'a>> {
    Box::pin(handle(client, ctx))
}

async fn handle(client: &DownloadClient, ctx: &DownloadContext) -> HandlerResult {
    log::info!("fallback: checking if publisher URL is a direct PDF: {}", ctx.publisher_url);

    // HEAD request to check Content-Type without downloading the body.
    if client.is_direct_pdf(&ctx.publisher_url).await {
        log::info!("fallback: publisher URL is a direct PDF, downloading...");
        return client
            .download(&ctx.publisher_url, &ctx.download_dir, &ctx.filename, &ctx.doi, &[])
            .await
            .map(Some)
            .ok_or_else(|| "fallback PDF download failed".to_string());
    }

    log::info!("fallback: publisher URL is not a direct PDF, giving up");
    Err("no handler matched and the URL is not a direct PDF link".to_string())
}
