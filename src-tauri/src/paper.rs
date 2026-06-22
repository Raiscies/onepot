use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inline the embedded CCF rank JSON. Requires the `once_cell` dependency.
use once_cell::sync::Lazy;

/// CCF rank lookup table: venue full name → rank ("A"|"B"|"C"|"P").
static CCF_RANK_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let json_str = include_str!("../resources/ccfrank/venue_to_rank.json");
    #[derive(Deserialize)]
    struct CcfEntry {
        rank: String,
        #[allow(dead_code)]
        abbr: String,
    }
    let raw: HashMap<String, CcfEntry> =
        serde_json::from_str(json_str).expect("Failed to parse venue_to_rank.json");
    raw.into_iter().map(|(k, v)| (k, v.rank)).collect()
});

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

    // source
    pub venue: Option<String>,
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

    // citation parse context
    #[serde(default)]
    pub raw_citation: String,

    // display status
    #[serde(default)]
    pub status: PaperStatus,
}

impl Paper {
    /// Merge non-empty fields from `other` into `self`.
    /// Only merges when both papers share the same DOI (or either DOI is empty).
    pub fn merge(&mut self, other: &Paper) {
        // Only merge papers that are confirmed to be the same work
        if self.doi.is_some() && other.doi.is_some() && self.doi != other.doi {
            return;
        }
        if self.title.is_none() {
            self.title = other.title.clone();
        }
        if self.authors.is_empty() && !other.authors.is_empty() {
            self.authors = other.authors.clone();
        }
        merge_opt!(self, other, year);
        merge_opt!(self, other, doi);
        merge_opt!(self, other, venue);
        merge_opt!(self, other, volume);
        merge_opt!(self, other, issue);
        merge_opt!(self, other, pages);
        merge_opt!(self, other, publisher);
        merge_opt!(self, other, url);
        merge_opt!(self, other, tldr);
        merge_opt!(self, other, abstract_);
        merge_opt!(self, other, citation_count);
        merge_opt!(self, other, ccf_rank);
        // merge_opt!(self, other, raw_citation);
    }

    pub fn has_doi(&self) -> bool {
        self.doi.as_ref().map_or(false, |d| !d.is_empty())
    }

    /// Resolve CCF rank from the paper's venue name.
    /// Sets `ccf_rank` to "A", "B", "C", "P" (preprint), or "N" (not found).
    /// If venue is empty, leaves ccf_rank unchanged.
    pub fn resolve_ccf_rank(&mut self) {
        let venue = match &self.venue {
            Some(v) if !v.trim().is_empty() => v.trim(),
            _ => return, // venue is empty, keep ccf_rank as-is
        };
        let rank = CCF_RANK_MAP.get(venue).cloned().unwrap_or_else(|| "N".to_string());
        self.ccf_rank = Some(rank);
    }
}

/// Frontend display state for a PaperCard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaperStatus {
    Ready,
    Searching,
    Error,
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
    pub error_msg: Option<String>,
}

impl ParseResult {
    /// Create a placeholder entry shown as a skeleton while searching.
    pub fn placeholder(index: usize, citation_index: Option<&str>, raw_citation: &str) -> Self {
        ParseResult {
            paper: Paper {
                status: PaperStatus::Searching,
                raw_citation: raw_citation.to_string(),
                ..Default::default()
            },
            index,
            citation_index: citation_index.map(|s| s.to_string()),
            error_msg: None,
        }
    }

    /// Create an error entry when citation parsing fails.
    pub fn error(index: usize, raw_citation: &str, error_msg: &str) -> Self {
        ParseResult {
            paper: Paper {
                status: PaperStatus::Error,
                raw_citation: raw_citation.to_string(),
                ..Default::default()
            },
            index,
            citation_index: None,
            error_msg: Some(error_msg.to_string()),
        }
    }

    /// Merge a search result into this card, with score determining priority.
    /// The higher-scored result is the primary; the lower fills in missing fields.
    pub fn apply_single_result(&mut self, new_result: &SearchResult) {
        // Compare scores: higher score wins as primary
        let (primary, secondary) = if new_result.score > 0.0 {
            // new result has a score; compare with current best
            // (DOI exact match = 1.0, bibliographic = 0.0)
            (&new_result.paper, &self.paper)
        } else {
            (&self.paper, &new_result.paper)
        };

        let mut merged = primary.clone();
        merged.merge(secondary);

        // Fall back to venue-based CCF lookup if no rank from search APIs
        if merged.ccf_rank.is_none() || merged.ccf_rank.as_deref() == Some("P") {
            merged.resolve_ccf_rank();
        }
        merged.status = PaperStatus::Ready;
        self.paper = merged;
    }

    /// Merge all search results (sorted by score desc) into this card, with the
    /// parse result as the lowest-priority fallback.
    #[deprecated(note = "use apply_single_result in a streaming loop instead")]
    pub fn apply_all_results(&mut self, results: &[SearchResult]) {
        let mut merged = results[0].paper.clone();
        for r in &results[1..] {
            merged.merge(&r.paper);
        }
        merged.merge(&self.paper);
        // Fall back to venue-based CCF lookup if no rank from search APIs
        if merged.ccf_rank.is_none() || merged.ccf_rank.as_deref() == Some("P") {
            merged.resolve_ccf_rank();
        }
        merged.status = PaperStatus::Ready;
        self.paper = merged;
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

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "lowercase")]
// pub enum DownloadStatus {
//     Pending,
//     Downloading,
//     Completed,
//     Failed,
// }

// /// A PDF download task tracked by the download engine.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DownloadTask {
//     pub paper: Paper,
//     pub url: String,
//     pub source: String,
//     pub status: DownloadStatus,
//     pub progress: f64,
//     pub error: Option<String>,
//     pub file_path: Option<String>,
// }

// impl DownloadTask {
//     pub fn new(paper: Paper, url: &str, source: &str) -> Self {
//         DownloadTask {
//             paper,
//             url: url.to_string(),
//             source: source.to_string(),
//             status: DownloadStatus::Pending,
//             progress: 0.0,
//             error: None,
//             file_path: None,
//         }
//     }
// }
