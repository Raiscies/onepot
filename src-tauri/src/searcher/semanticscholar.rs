use crate::paper::{Paper, SearchResult};
use crate::paper_search::Searcher;
use async_trait::async_trait;
use serde::Deserialize;

const API_URL: &str = "https://api.semanticscholar.org/graph/v1/paper/search/match";
const FIELDS: &str = "title,authors,year,externalIds,publicationVenue,openAccessPdf";

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
    publication_venue: Option<SSVenue>,
    open_access_pdf: Option<SSOpenAccessPdf>,
}

#[derive(Debug, Deserialize)]
struct SSAuthor {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SSExternalIds {
    #[serde(rename = "DOI")]
    doi: Option<String>,
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

pub struct SemanticScholarSearcher {
    client: reqwest::Client,
}

impl SemanticScholarSearcher {
    pub fn new() -> Self {
        SemanticScholarSearcher {
            client: reqwest::Client::new(),
        }
    }

    fn map_paper(&self, ss: &SSPaper) -> Paper {
        let authors: Vec<String> = ss.authors.iter().map(|a| a.name.clone()).collect();

        let doi = ss
            .external_ids
            .as_ref()
            .and_then(|ids| ids.doi.clone());

        let venue = ss.publication_venue.as_ref();
        let journal = venue.and_then(|v| v.name.clone());
        let volume = venue.and_then(|v| v.volume.clone());
        let pages = venue.and_then(|v| v.pages.clone());

        let url = doi
            .as_ref()
            .map(|d| format!("https://doi.org/{d}"));

        Paper {
            title: ss.title.clone(),
            authors,
            year: ss.year,
            doi,
            journal,
            volume,
            issue: None,
            pages,
            publisher: None,
            url,
            ..Default::default()
        }
    }

    async fn search_by_doi(&self, doi: &str) -> Option<SearchResult> {
        self.query_api(&format!("DOI:{doi}")).await
    }

    async fn search_by_title(&self, title: &str) -> Option<SearchResult> {
        self.query_api(title).await
    }

    async fn query_api(&self, query: &str) -> Option<SearchResult> {
        let resp = self
            .client
            .get(API_URL)
            .query(&[("query", query), ("limit", "3"), ("fields", FIELDS)])
            .send()
            .await
            .ok()?;

        let body: SSResponse = resp.json().await.ok()?;
        let data = body.data?;
        let best = data.first()?;
        let matched = self.map_paper(best);

        let download_url = best
            .open_access_pdf
            .as_ref()
            .and_then(|oa| oa.url.clone());

        Some(SearchResult {
            paper: matched,
            source: self.name().to_string(),
            score: 0.0,
            download_url,
            available: true,
            error: None,
        })
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
