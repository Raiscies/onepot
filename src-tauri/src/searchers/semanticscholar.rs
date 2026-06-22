use crate::paper::{Paper, SearchResult};
use crate::paper_search::Searcher;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use once_cell::sync::Lazy;
use regex::Regex;

const API_URL: &str = "https://api.semanticscholar.org/graph/v1/paper/search/match";
const FIELDS: &str = "title,authors,year,externalIds,publicationVenue,url,openAccessPdf,abstract,tldr,citationCount";

/// Regex to strip trailing digits from DBLP URIs.
static FORMAT_DBLP_URI: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+/\w+)(?:/[\w\-]+)?$").unwrap());
static CCF_RANK_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let json_str = include_str!("../../resources/ccfrank/dblp_uri_to_rank.json");
    #[derive(Deserialize)]
    struct CcfEntry {
        rank: String,
        #[allow(dead_code)]
        venue: String,
        #[allow(dead_code)]
        abbr: String,
    }
    let raw: HashMap<String, CcfEntry> =
        serde_json::from_str(json_str).expect("Failed to parse dblp_uri_to_rank.json");
    raw.into_iter().map(|(k, v)| (k, v.rank)).collect()
});
#[derive(Debug, Deserialize)]
struct SSResponse {
    data: Option<Vec<SSPaper>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SSPaper {
    title: Option<String>,
    #[serde(default)]
    authors: Vec<SSAuthor>,
    year: Option<i32>,
    external_ids: Option<SSExternalIds>,
    #[serde(default)]
    url: Option<String>,
    publication_venue: Option<SSVenue>,
    open_access_pdf: Option<SSOpenAccessPdf>,
    #[serde(default, rename = "abstract")]
    abstract_: Option<String>,
    tldr: Option<SSTldr>,
    #[serde(default)]
    citation_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct SSAuthor {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SSExternalIds {
    #[serde(rename = "DOI")]
    doi: Option<String>,
    #[serde(rename = "ArXiv")]
    arxiv: Option<String>,
    #[serde(rename = "DBLP")]
    dblp: Option<String>
}

#[derive(Debug, Deserialize)]
struct SSVenue {
    name: Option<String>,
    volume: Option<String>,
    pages: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SSOpenAccessPdf {
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SSTldr {
    text: Option<String>,
}

pub struct SemanticScholarSearcher {
    client: reqwest::Client,
    /// Rate limiter: max 2 requests per second.
    rate_limiter: Mutex<Option<Instant>>,
}

impl SemanticScholarSearcher {
    pub fn new() -> Self {
        SemanticScholarSearcher {
            client: reqwest::Client::builder()
                .no_proxy()
                .build()
                .expect("Failed to build Semantic Scholar HTTP client"),
            rate_limiter: Mutex::new(None),
        }
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

    
    /// Look up CCF rank by DBLP URI. Trailing digits are stripped before lookup
    /// since Semantic Scholar appends them (e.g. /journals/tocs/tocs123 → /journals/tocs/tocs).
    fn resolve_ccf_rank(&self, ss: &SSPaper) -> Option<String> {
        let dblp_uri = ss
            .external_ids
            .as_ref()
            .and_then(|ids| ids.dblp.clone());
        let key = dblp_uri.as_deref().map(|uri| FORMAT_DBLP_URI.replace(uri, "/$1").to_string());
        let rank = key.as_deref().and_then(|k| CCF_RANK_MAP.get(k).cloned());
        log::debug!(
            "CCF lookup: dblp_uri={:?} -> key={:?} -> rank={:?}",
            dblp_uri, key, rank
        );
        if dblp_uri.is_some() {
            Some(rank.unwrap_or_else(|| "N".to_string()))
        } else {
            None
        }
    }

    fn map_paper(&self, ss: &SSPaper) -> Paper {
        let authors: Vec<String> = ss.authors.iter().map(|a| a.name.clone()).collect();

        let doi = ss
            .external_ids
            .as_ref()
            .and_then(|ids| ids.doi.clone());

        let arxiv_id = ss
            .external_ids
            .as_ref()
            .and_then(|ids| ids.arxiv.clone());

        let venue = ss.publication_venue.as_ref();
        let venue_name = venue.and_then(|v| v.name.clone());
        let volume = venue.and_then(|v| v.volume.clone());
        let pages = venue.and_then(|v| v.pages.clone());

        let url = doi
            .as_ref()
            .map(|d| format!("https://doi.org/{d}"))
            .or_else(|| arxiv_id.as_ref().map(|id| format!("https://arxiv.org/abs/{id}")))
            .or_else(|| ss.url.clone());

        let ccf_rank = self.resolve_ccf_rank(ss);

        Paper {
            title: ss.title.clone(),
            authors,
            year: ss.year,
            doi,
            venue: venue_name,
            volume,
            issue: None,
            pages,
            publisher: None,
            url,
            tldr: ss.tldr.as_ref().and_then(|t| t.text.clone()),
            abstract_: ss.abstract_.clone(),
            citation_count: ss.citation_count,
            ccf_rank,
            ..Default::default()
        }
    }

    async fn search_by_doi(&self, doi: &str) -> Option<SearchResult> {
        self.rate_limit().await;
        self.query_api(&format!("DOI:{doi}")).await
    }

    async fn search_by_title(&self, title: &str) -> Option<SearchResult> {
        self.rate_limit().await;
        self.query_api(title).await
    }

    async fn query_api(&self, query: &str) -> Option<SearchResult> {
        for attempt in 0..3 {
            let resp = match self
                .client
                .get(API_URL)
                .query(&[("query", query), ("limit", "3"), ("fields", FIELDS)])
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("SemanticScholar request failed for '{query}': {e}");
                    return None;
                }
            };

            if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let delay = Duration::from_secs(1u64 << attempt);
                log::warn!("SemanticScholar 429 for '{query}', retrying in {delay:?} (attempt {})", attempt + 1);
                sleep(delay).await;
                continue;
            }

            let body: SSResponse = match resp.json().await {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("SemanticScholar JSON parse failed for '{query}': {e}");
                    return None;
                }
            };
        let data = match body.data {
            Some(d) => d,
            None => {
                log::warn!("SemanticScholar returned no data for '{query}'");
                return None;
            }
        };
        let best = match data.first() {
            Some(b) => b,
            None => {
                log::warn!("SemanticScholar returned empty data array for '{query}'");
                return None;
            }
        };
        let matched = self.map_paper(best);

        let download_url = best
            .open_access_pdf
            .as_ref()
            .and_then(|oa| oa.url.clone());

        return Some(SearchResult {
                paper: matched,
                source: self.name().to_string(),
                score: 0.8,
                download_url,
                available: true,
                error: None,
            })
        }
        log::warn!("SemanticScholar exhausted retries for '{query}'");
        None
    }

}

#[async_trait]
impl Searcher for SemanticScholarSearcher {
    fn name(&self) -> &str {
        "SemanticScholar"
    }

    async fn search(&self, paper: &Paper) -> Option<SearchResult> {
        // try DOI first, fall back to title
        if let Some(doi) = &paper.doi {
            if let Some(result) = self.search_by_doi(doi).await {
                return Some(result);
            }
        }
        self.search_by_title(paper.title.as_ref()?).await
    }
}
