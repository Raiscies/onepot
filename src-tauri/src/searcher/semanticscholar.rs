use crate::paper::{Paper, SearchResult};
use crate::paper_search::Searcher;
use async_trait::async_trait;
use serde::Deserialize;

const API_URL: &str = "https://api.semanticscholar.org/graph/v1/paper/search/match";

#[derive(Debug, Deserialize)]
struct S2Response {
    data: Option<Vec<S2Paper>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct S2Paper {
    title: Option<String>,
    #[serde(default)]
    authors: Vec<S2Author>,
    year: Option<i32>,
    external_ids: Option<S2ExternalIds>,
    publication_venue: Option<S2Venue>,
}

#[derive(Debug, Deserialize)]
struct S2Author {
    name: String,
}

#[derive(Debug, Deserialize)]
struct S2ExternalIds {
    #[serde(rename = "DOI")]
    doi: Option<String>,
}

#[derive(Debug, Deserialize)]
struct S2Venue {
    name: Option<String>,
    volume: Option<String>,
    pages: Option<String>,
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

    fn map_paper(&self, s2: &S2Paper) -> Paper {
        let authors: Vec<String> = s2.authors.iter().map(|a| a.name.clone()).collect();

        let doi = s2
            .external_ids
            .as_ref()
            .and_then(|ids| ids.doi.clone());

        let journal = s2
            .publication_venue
            .as_ref()
            .and_then(|v| v.name.clone());

        let volume = s2
            .publication_venue
            .as_ref()
            .and_then(|v| v.volume.clone());

        let pages = s2
            .publication_venue
            .as_ref()
            .and_then(|v| v.pages.clone());

        let url = doi
            .as_ref()
            .map(|d| format!("https://doi.org/{d}"));

        Paper {
            title: s2.title.clone(),
            authors,
            year: s2.year,
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
}

#[async_trait]
impl Searcher for SemanticScholarSearcher {
    fn name(&self) -> &str {
        "SemanticScholar"
    }

    async fn search(&self, paper: &Paper) -> Option<SearchResult> {
        let query = paper.title.as_ref()?;

        let resp = self
            .client
            .get(API_URL)
            .query(&[
                ("query", query.as_str()),
                ("limit", "3"),
                (
                    "fields",
                    "title,authors,year,externalIds,publicationVenue",
                ),
            ])
            .send()
            .await
            .ok()?;

        let body: S2Response = resp.json().await.ok()?;
        let data = body.data?;
        let best = data.first()?;
        let matched = self.map_paper(best);

        Some(SearchResult {
            paper: matched,
            source: self.name().to_string(),
            score: 0.0,
            download_url: None,
            available: true,
            error: None,
        })
    }
}
