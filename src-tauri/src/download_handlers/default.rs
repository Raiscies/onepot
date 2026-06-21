use crate::download::{DefaultHandlerParam, DownloadClient, DownloadContext, HandlerResult, ScrapedValue};

/// How to extract a value from a matched HTML element.
#[derive(Debug, Clone)]
pub(crate) enum ExtractMode {
    Html,
    Attr(String),
    Text,
}

/// Data-driven default handler: resolve URL from template + origin + scrape, then download.
pub(crate) async fn default_handler(
    client: &DownloadClient,
    ctx: &DownloadContext,
    param: &DefaultHandlerParam,
) -> HandlerResult {
    let final_url = resolve_download_url(client, ctx, param)
        .await
        .ok_or_else(|| "failed to resolve download URL".to_string())?;

    log::info!("Download resolved URL for doi={}: {}", ctx.doi, final_url);

    client
        .download(&final_url, &ctx.download_dir, &ctx.filename, &ctx.doi, &[])
        .await
        .map(Some)
        .ok_or_else(|| "download failed".to_string())
}

// ── URL resolution ──

pub(crate) async fn resolve_download_url(
    client: &DownloadClient,
    ctx: &DownloadContext,
    param: &DefaultHandlerParam,
) -> Option<String> {
    let url_template = &param.url_template;
    if url_template.is_empty() {
        return None;
    }

    let origin_caps: Vec<String> = param
        .origin_regex
        .as_ref()
        .and_then(|re| re.captures(&ctx.publisher_url))
        .map(|caps| {
            (0..caps.len())
                .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect()
        })
        .unwrap_or_default();

    let scrape_caps: Vec<String> = match &param.scraped {
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
    log::debug!("content: {:?}", document.html());
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
