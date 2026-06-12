use crate::citation_parse::{parse_single, split_citations, set_runner_path};
use crate::paper::{Paper, ParseResult, PaperStatus};
use crate::paper_search::SearchService;
use crate::searcher::semanticscholar::SemanticScholarSearcher;
use crate::APP;
use log::{info, warn};
use tauri::Manager;

/// Full citation search pipeline:
/// Phase 1: emit captured text + placeholder cards
/// Phase 2: emit AnyStyle-parsed metadata per card
/// Phase 3: emit Semantic Scholar enriched data per card
#[tauri::command]
pub async fn citation_search(text: String) -> Result<(), String> {
    let app = APP.get().ok_or("App handle not initialized")?;

    // Init runner path if not set
    let _ = set_runner_path("src-tauri/resources/anystyle/runner.rb");

    // Open the citation window
    crate::window::open_citation_window();

    // Split citations
    let segments = split_citations(&text);
    if segments.is_empty() {
        warn!("No citations found in text");
        return Err("No citations found".to_string());
    }

    let total = segments.len();
    info!("Citation search: split into {total} segments");

    // Phase 1: emit initial data (captured text + placeholders)
    let placeholders: Vec<ParseResult> = segments
        .iter()
        .enumerate()
        .map(|(i, (idx, _))| ParseResult::placeholder(i, idx.as_deref()))
        .collect();

    let initial = serde_json::json!({
        "papers": placeholders,
        "captured_text": text,
        "total": total,
    });
    app.emit_all("citation_init", initial.to_string())
        .map_err(|e| format!("Failed to emit citation_init: {e}"))?;

    // Phase 2 & 3: parse and search each citation in parallel
    for (i, (citation_index, citation_text)) in segments.into_iter().enumerate() {
        let app = app.clone();

        tauri::async_runtime::spawn(async move {
            // Phase 2: parse via AnyStyle
            let mut result = parse_single(&citation_text, i, citation_index.as_deref());

            if !matches!(result.paper.status, PaperStatus::Error(_)) {
                // mark as searching while we enrich
                result.paper.status = PaperStatus::Searching;

                emit_update(&app, i, "parsed", &result);

                // Phase 3: enrich with Semantic Scholar
                let mut search_service = SearchService::new();
                search_service.add(Box::new(SemanticScholarSearcher::new()));
                let search_results = search_service.search_all(&result.paper).await;

                if let Some(best) = search_results.first() {
                    result.apply_search_result(&best.paper);
                } else {
                    result.paper.status = PaperStatus::Ready;
                }

                emit_update(&app, i, "enriched", &result);
            } else {
                emit_update(&app, i, "error", &result);
            }
        });
    }

    Ok(())
}

fn emit_update(app: &tauri::AppHandle, index: usize, phase: &str, data: &ParseResult) {
    let payload = serde_json::json!({
        "index": index,
        "phase": phase,
        "data": data,
    });
    let _ = app.emit_all("citation_update", payload.to_string());
}
