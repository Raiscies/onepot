use crate::paper::{Paper, ParseResult};
use regex::Regex;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::sync::Mutex;

/// Path to the Ruby binary (default: "ruby").
static RUBY_BIN: Mutex<Option<String>> = Mutex::new(None);

/// Path to the Ruby AnyStyle runner script.
static RUNNER_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Split a multi-citation text into individual citation segments.
/// Returns a list of `(citation_index, citation_text)` pairs.
pub fn split_citations(text: &str) -> Vec<(Option<String>, String)> {
    let linebreak_re = Regex::new(r"[\n\r\f]").unwrap();
    let bracket_re = Regex::new(r"\[[\w\+]{1,8}\]").unwrap();

    // Step 1: merge lines with smart space insertion
    let lines: Vec<&str> = linebreak_re.split(text).collect();
    let mut merged = String::new();
    let mut should_insert_space = false;
    for line in lines {
        let line = line.trim_start_matches(' ');
        if line.is_empty() {
            continue;
        }
        if should_insert_space || line.starts_with(|c: char| c.is_alphabetic()) {
            merged.push(' ');
        }
        should_insert_space = line.ends_with(|c: char| c.is_alphabetic());
        merged.push_str(line);
    }

    if merged.is_empty() {
        return vec![];
    }

    // Step 2: split by bracket markers like [1], [Tho00a]
    let mut citations = Vec::new();
    let mut last_match: Option<regex::Match> = None;

    for m in bracket_re.find_iter(&merged) {
        if last_match.is_none() {
            let content = merged[0..m.start()].trim().to_string();
            if !content.is_empty() {
                citations.push((None, content));
            }
        } else {
            let prev = last_match.unwrap();
            let content = merged[prev.end()..m.start()].trim_end().to_string();
            let index = prev.as_str();
            // strip leading [ and trailing ]
            let index = &index[1..index.len() - 1];
            citations.push((Some(index.to_string()), content));
        }
        last_match = Some(m);
    }

    // Step 3: handle the last segment
    if let Some(last) = last_match {
        let index = last.as_str();
        let index = &index[1..index.len() - 1];
        let content = merged[last.end()..].to_string();
        citations.push((Some(index.to_string()), content));
    } else {
        citations.push((None, merged));
    }

    citations
}

/// Parse a single citation string via Ruby AnyStyle subprocess.
pub fn parse_single(
    citation: &str,
    index: usize,
    citation_index: Option<&str>,
) -> ParseResult {
    let result = do_parse(citation);
    match result {
        Ok(paper) => ParseResult {
            paper,
            index,
            citation_index: citation_index.map(|s| s.to_string()),
            raw_citation: Some(citation.to_string()),
            error_msg: None,
        },
        Err(err_msg) => ParseResult::error(index, citation, &err_msg),
    }
}

fn do_parse(citation: &str) -> Result<Paper, String> {
    let runner = get_runner_path()?;
    let ruby = get_ruby_bin();

    let mut output = Command::new(ruby)
        .arg(&runner)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ruby: {e}"))?;

    // Write citation to stdin
    if let Some(mut stdin) = output.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(citation.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {e}"))?;
    }

    let output = output
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on ruby: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AnyStyle failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout.trim();
    if stdout.is_empty() {
        return Err("AnyStyle returned empty output".to_string());
    }

    let items: Value = serde_json::from_str(stdout)
        .map_err(|e| format!("Failed to parse AnyStyle JSON: {e}"))?;

    // AnyStyle returns an array; take the first element
    let item = match &items {
        Value::Array(arr) if !arr.is_empty() => &arr[0],
        _ => &items,
    };
    let paper = parse_anystyle_item(item)?;

    // AnyStyle may return valid JSON but with no useful data
    if paper.title.is_none() {
        return Err("AnyStyle returned empty title".to_string());
    }

    Ok(paper)
}

/// Set the path to the Ruby binary. Defaults to "ruby" if empty.
pub fn set_ruby_bin(path: &str) {
    let bin = if path.is_empty() { "ruby".to_string() } else { path.to_string() };
    *RUBY_BIN.lock().unwrap() = Some(bin);
}

/// Set the path to the Ruby AnyStyle runner script.
pub fn set_runner_path(path: &str) {
    *RUNNER_PATH.lock().unwrap() = Some(path.to_string());
}

fn get_ruby_bin() -> String {
    RUBY_BIN.lock().unwrap().clone().unwrap_or_else(|| "ruby".to_string())
}

fn get_runner_path() -> Result<String, String> {
    RUNNER_PATH
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| "AnyStyle runner path not set".to_string())
}

/// Helper: get the first element of an array, or the value itself if not an array.
fn get_first(value: &Value) -> Option<&Value> {
    match value {
        Value::Array(arr) => arr.first(),
        _ => Some(value),
    }
}

/// Helper: get string from a JSON value.
fn as_str(value: &Value) -> Option<&str> {
    value.as_str()
}

/// Map a string field from AnyStyle CSL JSON to a Paper Option field.
macro_rules! map_str_field {
    ($item:expr, $paper:expr, $key:literal, $field:ident) => {
        if let Some(v) = $item.get($key).and_then(get_first).and_then(as_str) {
            $paper.$field = Some(v.to_string());
        }
    };
}

/// Map an AnyStyle CSL JSON item to a Paper.
fn parse_anystyle_item(item: &Value) -> Result<Paper, String> {
    let mut paper = Paper::default();

    // authors
    if let Some(authors) = item.get("author").and_then(|a| a.as_array()) {
        for a in authors {
            if let Some(family) = a.get("family").and_then(as_str) {
                let given = a.get("given").and_then(as_str).unwrap_or("");
                let name = if given.is_empty() {
                    family.to_string()
                } else {
                    format!("{given} {family}")
                };
                paper.authors.push(name);
            } else if let Some(name) = a.as_str() {
                paper.authors.push(name.to_string());
            }
        }
    }

    // year
    if let Some(issued) = item.get("issued").or(item.get("date")) {
        if let Some(s) = issued.as_str() {
            if let Some(cap) = Regex::new(r"\b(19|20)\d{2}\b").unwrap().find(s) {
                paper.year = cap.as_str().parse().ok();
            }
        } else if let Some(date_parts) = issued
            .get("date-parts")
            .and_then(|dp| dp.as_array())
            .and_then(|arr| arr.first())
            .and_then(|inner| inner.as_array())
        {
            if let Some(y) = date_parts.first().and_then(|v| v.as_i64()) {
                paper.year = Some(y as i32);
            }
        }
    }

    // title
    let title = get_first(item.get("title").unwrap_or(&Value::Null))
        .and_then(as_str)
        .map(|s| s.to_string());
    if let Some(t) = title {
        paper.title = Some(t);
    } else if let Some(url) = item.get("URL").and_then(as_str) {
        paper.title = Some(url.to_string());
    }

    map_str_field!(item, paper, "container-title", journal);
    map_str_field!(item, paper, "volume", volume);
    map_str_field!(item, paper, "issue", issue);
    map_str_field!(item, paper, "page", pages);
    map_str_field!(item, paper, "publisher", publisher);

    // URL, strip trailing punctuation
    if let Some(u) = item.get("URL").and_then(get_first).and_then(as_str) {
        paper.url = Some(u.trim_end_matches(&['.', ',', ';']).to_string());
    }

    // DOI
    if let Some(d) = item.get("DOI").and_then(get_first).and_then(as_str) {
        paper.doi = Some(d.to_string());
        if paper.url.is_none() {
            paper.url = Some(format!("https://doi.org/{d}"));
        }
    }

    Ok(paper)
}
