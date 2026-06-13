use crate::paper::{Paper, SearchResult};
use async_trait::async_trait;

/// A literature database searcher.
#[async_trait]
pub trait Searcher: Send + Sync {
    /// Human-readable name of this searcher.
    fn name(&self) -> &str;

    /// Search for a paper. Returns None if not found.
    async fn search(&self, paper: &Paper) -> Option<SearchResult>;
}

/// Holds multiple searchers and runs them in parallel.
pub struct SearchService {
    searchers: Vec<Box<dyn Searcher>>,
}

impl SearchService {
    /// Create a new SearchService with all registered searchers.
    pub fn new() -> Self {
        SearchService {
            searchers: crate::searchers::all_searchers(),
        }
    }

    pub fn add(&mut self, searcher: Box<dyn Searcher>) {
        self.searchers.push(searcher);
    }

    /// Run all searchers in parallel, returning results for those that found a match.
    pub async fn search_all(&self, paper: &Paper) -> Vec<SearchResult> {
        let futures: Vec<_> = self
            .searchers
            .iter()
            .map(|s| s.search(paper))
            .collect();

        let results = futures::future::join_all(futures).await;

        results
            .into_iter()
            .flatten()
            .collect()
    }
}
