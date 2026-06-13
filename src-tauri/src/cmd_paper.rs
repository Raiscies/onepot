use crate::citation_parse::{parse_single, set_ruby_bin, set_runner_path, split_citations};
use crate::paper::{ParseResult, PaperStatus};
use crate::paper_search::SearchService;
use crate::CitationResultsWrapper;
use log::info;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use tauri::Manager;

/// Shared search service, initialized on first use.
static SEARCH_SERVICE: OnceLock<SearchService> = OnceLock::new();

/// Incremented on each search to cancel stale tasks.
static CITATION_SEARCH_ID: AtomicU64 = AtomicU64::new(0);

/// Call once at startup and when ruby path config changes.
#[tauri::command]
pub fn reinit_ruby() {
    let ruby_path = crate::config::get("citation_ruby_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();
    set_ruby_bin(&ruby_path);
    let _ = set_runner_path("resources/anystyle/runner.rb");
}

/// Full pipeline for a citation search.
pub fn run_citation_pipeline(app: &tauri::AppHandle, window: &tauri::Window, text: &str) {
    SEARCH_SERVICE.get_or_init(SearchService::new);
    let my_id = CITATION_SEARCH_ID.fetch_add(1, Ordering::SeqCst) + 1;
    info!("citation_search #{my_id}: starting");

    let results_wrapper: tauri::State<CitationResultsWrapper> = app.state();

    let placeholders = spawn_tasks(app, window, text, my_id);

    *results_wrapper.0.lock().unwrap() = placeholders;
}

/// Emit a citation_update event for the given card.
fn emit_update(window: &tauri::Window, index: usize, phase: &str, data: &ParseResult) {
    let payload = serde_json::json!({
        "index": index,
        "phase": phase,
        "data": data,
    });
    let _ = window.emit("citation_update", payload.to_string());
}

/// Split citations, emit init, spawn per-segment parse+search tasks.
fn spawn_tasks(
    app: &tauri::AppHandle,
    window: &tauri::Window,
    text: &str,
    search_id: u64,
) -> Vec<ParseResult> {
    let segments = split_citations(text);
    let total = segments.len();

    let placeholders: Vec<ParseResult> = segments
        .iter()
        .enumerate()
        .map(|(i, (idx, raw))| {
            let mut p = ParseResult::placeholder(i, idx.as_deref());
            p.raw_citation = Some(raw.clone());
            p
        })
        .collect();

    // Emit init
    let init_payload = serde_json::json!({
        "papers": &placeholders,
        "captured_text": text,
        "total": total,
    });
    let _ = window.emit("citation_init", init_payload.to_string());
    info!("citation_search #{search_id}: emitted init, {total} papers");

    // Spawn per-segment tasks
    for (i, (citation_index, citation_text)) in segments.into_iter().enumerate() {
        let w = window.clone();
        let a = app.clone();
        tauri::async_runtime::spawn(async move {
            run_stages(&w, &a, i, &citation_text, citation_index.as_deref(), search_id).await;
        });
    }

    placeholders
}

async fn run_stages(
    window: &tauri::Window,
    app: &tauri::AppHandle,
    index: usize,
    citation_text: &str,
    citation_index: Option<&str>,
    search_id: u64,
) {
    if is_cancelled(search_id) { return; }

    // Stage 1: parse
    let mut result = parse_single(citation_text, index, citation_index);
    if is_cancelled(search_id) { return; }

    // If parsing failed or title is empty, report error and skip enrichment
    let is_parse_error = matches!(result.paper.status, PaperStatus::Error);
    if is_parse_error || result.paper.title.is_none() {
        if !is_parse_error {
            result.paper.status = PaperStatus::Error;
            result.error_msg = Some("AnyStyle returned empty title".to_string());
        }
        emit_update(window, index, "parsed", &result);
        write_shared_result(app, index, &result);
        info!("citation_search #{search_id}: paper #{index} parse failed");
        return;
    }

    result.paper.status = PaperStatus::Ready;
    emit_update(window, index, "parsed", &result);
    write_shared_result(app, index, &result);
    info!("citation_search #{search_id}: paper #{index} parsed");

    // Stage 2: enrich (only if parse produced usable metadata)
    if result.paper.has_doi() || result.paper.title.is_some() {
        if is_cancelled(search_id) { return; }
        let search_service = SEARCH_SERVICE.get().unwrap();
        if let Some(best) = search_service.search_all(&result.paper).await.first() {
            result.apply_search_result(&best.paper);
            info!("citation_search #{search_id}: paper #{index} enriched data: {:?}", result.paper);
        }
        if is_cancelled(search_id) { return; }
        emit_update(window, index, "enriched", &result);
        write_shared_result(app, index, &result);
        info!("citation_search #{search_id}: paper #{index} enriched");
    }
}

fn is_cancelled(search_id: u64) -> bool {
    CITATION_SEARCH_ID.load(Ordering::SeqCst) != search_id
}

fn write_shared_result(app: &tauri::AppHandle, index: usize, data: &ParseResult) {
    let results: tauri::State<CitationResultsWrapper> = app.state();
    let mut guard = results.0.lock().unwrap();
    if index < guard.len() {
        guard[index] = data.clone();
    }
}

