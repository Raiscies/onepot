use crate::paper::{Paper, SearchResult};
use crate::paper_search::Searcher;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Base URL for the public Crossref REST API.
const API_URL: &str = "https://api.crossref.org";
/// Fields requested in every search query (minimises response size).
const SELECT_FIELDS: &str = "DOI,title,author,container-title,issued,volume,page,publisher,abstract,type,URL";

/// Precompiled regex: strip all XML/HTML tags including JATS namespace prefixes.
static JATS_TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"</?[a-zA-Z_:][^>]*>").unwrap());

// ── Text cleaning helpers ──

/// Decode common HTML character entities in-place.
/// Handles named entities (`&amp;`, `&lt;`, etc.) and numeric entities (`&#39;`, `&#x2F;`).
fn unescape_html(text: &str) -> String {
    let mut result = text.to_string();
    // Named entities — ordered by frequency in Crossref metadata.
    for (entity, ch) in &[
        ("&amp;", '&'),
        ("&lt;", '<'),
        ("&gt;", '>'),
        ("&quot;", '"'),
        ("&#39;", '\''),
        ("&apos;", '\''),
    ] {
        result = result.replace(entity, &ch.to_string());
    }
    result
}

/// Strip JATS/XML tags and normalise whitespace.
/// `<jats:p>text</jats:p>` → `text`.  Also handles nested tags.
fn strip_jats_xml(raw: &str) -> String {
    let stripped = JATS_TAG_RE.replace_all(raw, " ");
    // Collapse consecutive whitespace into single spaces and trim.
    let collapsed: String = stripped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    collapsed
}

/// Clean a Crossref text field: unescape HTML entities.
/// Returns `None` if the value is already `None` or empty after cleaning.
fn clean_text(value: Option<&str>) -> Option<String> {
    let cleaned = unescape_html(value?);
    let trimmed = cleaned.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Clean Crossref abstract: unescape entities, strip JATS XML tags.
fn clean_abstract(raw: Option<&str>) -> Option<String> {
    let text = unescape_html(raw?);
    let stripped = strip_jats_xml(&text);
    let trimmed = stripped.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

// ── Response deserialisation ──

/// Top-level response from `/works` (search).
#[derive(Debug, Deserialize)]
struct CrResponse {
    message: CrMessage,
}

#[derive(Debug, Deserialize)]
struct CrMessage {
    #[serde(default)]
    items: Vec<CrWork>,
}

/// Top-level response from `/works/{doi}` (single work lookup).
#[derive(Debug, Deserialize)]
struct CrSingleResponse {
    message: CrWork,
}

/// A single work ("paper") from Crossref.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CrWork {
    #[serde(rename = "DOI", default)]
    doi: Option<String>,
    #[serde(default)]
    title: Vec<String>,
    #[serde(default)]
    author: Vec<CrAuthor>,
    /// Journal / conference name.
    #[serde(default, rename = "container-title")]
    container_title: Vec<String>,
    #[serde(default)]
    issued: Option<CrDateParts>,
    #[serde(default)]
    volume: Option<String>,
    #[serde(default)]
    page: Option<String>,
    #[serde(default)]
    publisher: Option<String>,
    /// Article abstract text.
    #[serde(default)]
    r#abstract: Option<String>,
    // #[serde(default, rename = "type")]
    // work_type: Option<String>,
    #[serde(rename = "URL", default)]
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrAuthor {
    #[serde(default)]
    given: Option<String>,
    #[serde(default)]
    family: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrDateParts {
    #[serde(rename = "date-parts")]
    date_parts: Vec<Vec<i32>>,
}

// ── Searcher ──

pub struct CrossrefSearcher {
    client: reqwest::Client,
    /// Polite pool email — when set, requests include `mailto={email}` for
    /// friendlier rate limits.
    email: Option<String>,
    /// Rate limiter: max 2 requests per second (500ms gap).
    rate_limiter: Mutex<Option<Instant>>,
}

impl CrossrefSearcher {
    pub fn new() -> Self {
        CrossrefSearcher {
            client: reqwest::Client::builder()
                .no_proxy()
                .build()
                .expect("Failed to build Crossref HTTP client"),
            email: None,
            rate_limiter: Mutex::new(None),
        }
    }

    /// Register an email address for the polite pool.
    ///
    /// Crossref's rate limits are more generous when a `mailto` parameter is
    /// present.  Use a contact address that you monitor — Crossref may reach
    /// out if your usage is excessive.
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Enforce max 2 requests per second (500ms gap).
    async fn rate_limit(&self) {
        let wait = {
            let mut last = self.rate_limiter.lock().unwrap();
            let now = Instant::now();
            let gap = last.map_or(Duration::ZERO, |prev| {
                Duration::from_millis(500).saturating_sub(now - prev)
            });
            *last = Some(now);
            gap
        };
        if !wait.is_zero() {
            sleep(wait).await;
        }
    }

    /// Build common query params used by every request.
    fn polite_param(&self) -> Vec<(&str, &str)> {
        if let Some(ref email) = self.email {
            vec![("mailto", email.as_str())]
        } else {
            Vec::new()
        }
    }

    /// Map one Crossref work into our uniform `Paper` struct.
    fn map_paper(work: &CrWork) -> Paper {
        let authors: Vec<String> = work
            .author
            .iter()
            .map(|a| {
                let name = match (&a.given, &a.family) {
                    (Some(g), Some(f)) => format!("{g} {f}"),
                    _ => a.name.clone().unwrap_or_default(),
                };
                unescape_html(&name)
            })
            .collect();

        let year = work
            .issued
            .as_ref()
            .and_then(|d| d.date_parts.first())
            .and_then(|parts| parts.first())
            .copied();

        let venue = work.container_title.first().map(|s| unescape_html(s));
        let doi = work.doi.clone();
        let url = work.url.clone().or_else(|| {
            doi.as_ref().map(|d| format!("https://doi.org/{d}"))
        });

        let paper = Paper {
            title: work.title.first().map(|s| unescape_html(s)),
            authors,
            year,
            doi,
            venue,
            volume: work.volume.as_deref().and_then(|v| clean_text(Some(v))),
            issue: None,
            pages: work.page.as_deref().and_then(|p| clean_text(Some(p))),
            publisher: work.publisher.as_deref().and_then(|p| clean_text(Some(p))),
            url: url.map(|u| unescape_html(&u)),
            abstract_: clean_abstract(work.r#abstract.as_deref()),
            ..Default::default()
        };
        // log::debug!("Crossref mapped Paper: {:?}", paper);
        paper
    }

    /// Search by DOI — exact match via `/works/{doi}`.
    async fn search_by_doi(&self, doi: &str) -> Option<SearchResult> {
        self.rate_limit().await;

        let url = format!("{API_URL}/works/{doi}");
        let polite = self.polite_param();

        log::debug!("Crossref GET {} (params: {:?})", url, polite);

        let resp = match self.client.get(&url).query(&polite).send().await {
            Ok(r) => r,
            Err(e) => {
                log::warn!("Crossref DOI lookup failed for '{doi}': {e}");
                return None;
            }
        };

        log::debug!("Crossref DOI response: status={}", resp.status());

        if !resp.status().is_success() {
            log::warn!("Crossref DOI lookup returned {} for '{doi}'", resp.status());
            return None;
        }

        let raw_text = match resp.text().await {
            Ok(t) => t,
            Err(e) => {
                log::warn!("Crossref DOI read body failed for '{doi}': {e}");
                return None;
            }
        };
        // log::debug!("Crossref DOI response body ({} bytes): {}", raw_text.len(), &raw_text[..raw_text.len().min(2000)]);

        let body: CrSingleResponse = match serde_json::from_str(&raw_text) {
            Ok(b) => b,
            Err(e) => {
                log::warn!("Crossref JSON parse failed for DOI '{doi}': {e}");
                return None;
            }
        };

        let paper = Self::map_paper(&body.message);
        Some(SearchResult {
            paper,
            source: "Crossref".to_string(),
            score: 1.0,
            download_url: None,
            available: true,
            error: None,
        })
    }

    /// Search by raw citation text — bibliographic query.
    async fn search_by_bibliographic(&self, raw_citation: &str) -> Option<SearchResult> {
        self.rate_limit().await;

let query = raw_citation.to_string();

        for attempt in 0..3 {
            let mut params: Vec<(&str, String)> = self
                .polite_param()
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            params.push(("query.bibliographic", query.clone()));
            params.push(("rows", "1".to_string()));
            params.push(("select", SELECT_FIELDS.to_string()));

            let req_url = format!("{API_URL}/works");
            log::debug!("Crossref GET {} (params: {:?})", req_url, params);

            let resp = match self.client.get(&req_url).query(&params).send().await {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("Crossref bibliographic search failed for '{query}': {e}");
                    return None;
                }
            };

            log::debug!("Crossref bibliographic response: status={}", resp.status());

            if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let delay = Duration::from_secs(1u64 << attempt);
                log::warn!("Crossref 429 for '{query}', retrying in {delay:?} (attempt {})", attempt + 1);
                sleep(delay).await;
                continue;
            }

            if !resp.status().is_success() {
                log::warn!("Crossref bibliographic search returned {} for '{query}'", resp.status());
                return None;
            }

            let raw_text = match resp.text().await {
                Ok(t) => t,
                Err(e) => {
                    log::warn!("Crossref bibliographic read body failed for '{query}': {e}");
                    return None;
                }
            };
            log::debug!("Crossref bibliographic response body ({} bytes): {}", raw_text.len(), &raw_text);

            let body: CrResponse = match serde_json::from_str(&raw_text) {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("Crossref JSON parse failed for '{query}': {e}");
                    return None;
                }
            };

            let best = match body.message.items.first() {
                Some(b) => b,
                None => {
                    log::warn!("Crossref returned no items for '{query}'");
                    return None;
                }
            };

            let paper = Self::map_paper(best);
            return Some(SearchResult {
                paper,
                source: "Crossref".to_string(),
                score: 0.9,
                download_url: None,
                available: true,
                error: None,
            });
        }

        log::warn!("Crossref exhausted retries for '{query}'");
        None
    }
}

#[async_trait]
impl Searcher for CrossrefSearcher {
    fn name(&self) -> &str {
        "Crossref"
    }

    async fn search(&self, paper: &Paper) -> Option<SearchResult> {
        // Try DOI first
        if let Some(doi) = &paper.doi {
            if let Some(result) = self.search_by_doi(doi).await {
                return Some(result);
            }
        }
        // Fall back to raw citation text
        let raw = &paper.raw_citation;
        self.search_by_bibliographic(raw).await
    }
}
