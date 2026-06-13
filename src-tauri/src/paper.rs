use serde::{Deserialize, Serialize};

/// Merges an Option field: if self is None and other is Some, copy from other.
macro_rules! merge_opt {
    ($self:expr, $other:expr, $field:ident) => {
        if $self.$field.is_none() && $other.$field.is_some() {
            $self.$field = $other.$field.clone();
        }
    };
}

/// Pure bibliographic metadata for a paper.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paper {
    // basic bibliographic fields
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub year: Option<i32>,
    pub doi: Option<String>,

    // source / venue
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub publisher: Option<String>,
    pub url: Option<String>,

    // enrichment fields (from search APIs)
    #[serde(default)]
    pub tldr: Option<String>,
    #[serde(default, rename = "abstract")]
    pub abstract_: Option<String>,
    #[serde(default)]
    pub citation_count: Option<i32>,
    #[serde(default)]
    pub ccf_rank: Option<String>,

    // display status
    #[serde(default)]
    pub status: PaperStatus,
}

impl Paper {
    /// Merge non-empty fields from `other` into `self`.
    pub fn merge(&mut self, other: &Paper) {
        if self.title.is_none() {
            self.title = other.title.clone();
        }
        if self.authors.is_empty() && !other.authors.is_empty() {
            self.authors = other.authors.clone();
        }
        merge_opt!(self, other, year);
        merge_opt!(self, other, doi);
        merge_opt!(self, other, journal);
        merge_opt!(self, other, volume);
        merge_opt!(self, other, issue);
        merge_opt!(self, other, pages);
        merge_opt!(self, other, publisher);
        merge_opt!(self, other, url);
        merge_opt!(self, other, tldr);
        merge_opt!(self, other, abstract_);
        merge_opt!(self, other, citation_count);
        merge_opt!(self, other, ccf_rank);
    }

    pub fn has_doi(&self) -> bool {
        self.doi.as_ref().map_or(false, |d| !d.is_empty())
    }
}

/// Frontend display state for a PaperCard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaperStatus {
    Ready,
    Searching,
    Error(String),
}

impl Default for PaperStatus {
    fn default() -> Self {
        PaperStatus::Ready
    }
}

/// Result of parsing a single citation: Paper with display context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub paper: Paper,
    // position index in the result list
    pub index: usize,
    // parsing context
    #[serde(default)]
    pub citation_index: Option<String>,
    #[serde(default)]
    pub raw_citation: Option<String>,
}

impl ParseResult {
    /// Create a placeholder entry shown as a skeleton while searching.
    pub fn placeholder(index: usize, citation_index: Option<&str>) -> Self {
        ParseResult {
            paper: Paper {
                status: PaperStatus::Searching,
                ..Default::default()
            },
            index,
            citation_index: citation_index.map(|s| s.to_string()),
            raw_citation: None,
        }
    }

    /// Create an error entry when citation parsing fails.
    pub fn error(index: usize, raw_citation: &str, error_msg: &str) -> Self {
        ParseResult {
            paper: Paper {
                status: PaperStatus::Error(error_msg.to_string()),
                ..Default::default()
            },
            index,
            citation_index: None,
            raw_citation: Some(raw_citation.to_string()),
        }
    }

    /// Merge search result into this card's paper, then mark ready.
    pub fn apply_search_result(&mut self, result: &Paper) {
        self.paper.merge(result);
        self.paper.status = PaperStatus::Ready;
    }
}

/// A search result returned by a literature database (Semantic Scholar, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub paper: Paper,
    pub source: String,
    pub score: f64,
    pub download_url: Option<String>,
    pub available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
}

/// A PDF download task tracked by the download engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub paper: Paper,
    pub url: String,
    pub source: String,
    pub status: DownloadStatus,
    pub progress: f64,
    pub error: Option<String>,
    pub file_path: Option<String>,
}

impl DownloadTask {
    pub fn new(paper: Paper, url: &str, source: &str) -> Self {
        DownloadTask {
            paper,
            url: url.to_string(),
            source: source.to_string(),
            status: DownloadStatus::Pending,
            progress: 0.0,
            error: None,
            file_path: None,
        }
    }
}
