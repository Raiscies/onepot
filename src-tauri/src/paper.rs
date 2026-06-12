use serde::{Deserialize, Serialize};

/// Paper metadata, serialized to JSON for the frontend.
/// The frontend handles all i18n / display formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // citation index marker (e.g. "[1]", "[Tho00a]")
    pub citation_index: Option<String>,

    // frontend state markers (not bibliographic data)
    #[serde(default)]
    pub _placeholder: Option<bool>,
    #[serde(default)]
    pub _index: Option<usize>,
    #[serde(default)]
    pub _error: Option<String>,
    #[serde(default)]
    pub _searching: Option<bool>,
}

impl Paper {
    /// Create a placeholder entry shown as a skeleton card while searching.
    pub fn placeholder(index: usize) -> Self {
        Paper {
            _placeholder: Some(true),
            _index: Some(index),
            _searching: Some(true),
            ..Default::default()
        }
    }

    /// Create an error entry when citation parsing fails.
    pub fn error(index: usize, error_msg: &str) -> Self {
        Paper {
            _placeholder: Some(false),
            _index: Some(index),
            _error: Some(error_msg.to_string()),
            _searching: Some(false),
            ..Default::default()
        }
    }

    /// Merge non-empty fields from `other` into `self`.
    /// Placeholder / error / searching markers are cleared after merge.
    pub fn merge(&mut self, other: &Paper) {
        // title: only overwrite if self has nothing meaningful
        if self.title.is_none() || self._placeholder == Some(true) || self._error.is_some() {
            if let Some(ref t) = other.title {
                self.title = Some(t.clone());
            }
        }
        if self.authors.is_empty() && !other.authors.is_empty() {
            self.authors = other.authors.clone();
        }
        if self.year.is_none() && other.year.is_some() {
            self.year = other.year;
        }
        if self.doi.is_none() && other.doi.is_some() {
            self.doi = other.doi.clone();
        }
        if self.journal.is_none() && other.journal.is_some() {
            self.journal = other.journal.clone();
        }
        if self.volume.is_none() && other.volume.is_some() {
            self.volume = other.volume.clone();
        }
        if self.issue.is_none() && other.issue.is_some() {
            self.issue = other.issue.clone();
        }
        if self.pages.is_none() && other.pages.is_some() {
            self.pages = other.pages.clone();
        }
        if self.publisher.is_none() && other.publisher.is_some() {
            self.publisher = other.publisher.clone();
        }
        if self.url.is_none() && other.url.is_some() {
            self.url = other.url.clone();
        }
        // clear transient markers after real data arrives
        self._placeholder = Some(false);
        self._searching = Some(false);
        self._error = None;
    }

    /// Returns true when the DOI field is present and non-empty.
    pub fn has_doi(&self) -> bool {
        self.doi.as_ref().map_or(false, |d| !d.is_empty())
    }
}

impl Default for Paper {
    fn default() -> Self {
        Paper {
            title: None,
            authors: vec![],
            year: None,
            doi: None,
            journal: None,
            volume: None,
            issue: None,
            pages: None,
            publisher: None,
            url: None,
            citation_index: None,
            _placeholder: None,
            _index: None,
            _error: None,
            _searching: None,
        }
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
