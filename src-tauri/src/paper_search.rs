use crate::paper::{Paper, SearchResult};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

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

    #[allow(unused)]
    pub fn add(&mut self, searcher: Box<dyn Searcher>) {
        self.searchers.push(searcher);
    }

    /// Return a Vec of boxed futures so the caller can process results as they
    /// complete (e.g. via `FuturesUnordered`).
    pub fn search_futures<'a>(
        &'a self,
        paper: &'a Paper,
    ) -> Vec<Pin<Box<dyn Future<Output = Option<SearchResult>> + Send + 'a>>> {
        self.searchers
            .iter()
            .map(|s| Box::pin(s.search(paper)) as Pin<Box<dyn Future<Output = Option<SearchResult>> + Send + 'a>>)
            .collect()
    }
}
