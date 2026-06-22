pub mod crossref;
pub mod semanticscholar;

use crate::paper_search::Searcher;

/// Create all registered searchers. Add new searchers here.
pub fn all_searchers() -> Vec<Box<dyn Searcher>> {
    vec![
        Box::new(semanticscholar::SemanticScholarSearcher::new()),
        Box::new(crossref::CrossrefSearcher::new()),
    ]
}
