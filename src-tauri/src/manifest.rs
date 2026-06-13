use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// DOI → filename mapping, persisted as a flat JSON object.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DownloadManifest {
    records: HashMap<String, String>,
}

impl DownloadManifest {
    /// Load manifest from a JSON file. Returns empty if file doesn't exist.
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Save manifest to a JSON file.
    pub fn save(&self, path: &Path) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }

    /// Look up a DOI and return the stored filename if the file exists.
    /// Returns None and cleans the stale record if the file is missing.
    pub fn get_and_clean(&mut self, doi: &str, storage_dir: &Path) -> Option<PathBuf> {
        let fname = self.records.get(doi)?;
        let path = storage_dir.join(fname);
        if path.exists() && path.metadata().map(|m| m.len() > 0).unwrap_or(false) {
            Some(path)
        } else {
            self.records.remove(doi);
            None
        }
    }

    /// Look up a DOI and return the stored filename if the file exists.
    pub fn get(&self, doi: &str, storage_dir: &Path) -> Option<PathBuf> {
        let fname = self.records.get(doi)?;
        let path = storage_dir.join(fname);
        if path.exists() && path.metadata().map(|m| m.len() > 0).unwrap_or(false) {
            Some(path)
        } else {
            None
        }
    }

    /// Register a download.
    pub fn set(&mut self, doi: &str, filename: &str) {
        self.records.insert(doi.to_string(), filename.to_string());
    }
}

/// Build a filename from a pattern using placeholder substitution.
/// Supported placeholders: {doi}, {title}, {author}, {pubdate}, {downdate}
pub fn build_filename(pattern: &str, doi: &str, meta: &PaperMeta) -> String {
    let safe_doi = doi.replace('/', "_");
    let title = sanitize(&meta.title).replace(char::is_whitespace, " ");
    let title = title.trim().chars().take(80).collect::<String>();
    let first_author = meta
        .authors
        .first()
        .map(|a| sanitize(a))
        .unwrap_or_default();
    let pubdate = meta
        .year
        .map(|y| y.to_string())
        .unwrap_or_default();
    let downdate = {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days_since_epoch = secs / 86400;
        format!("{days_since_epoch}")
    };

    let result = pattern
        .replace("{doi}", &safe_doi)
        .replace("{title}", &title)
        .replace("{author}", &first_author)
        .replace("{pubdate}", &pubdate)
        .replace("{downdate}", &downdate);

    let result = result.trim_matches(&['.', ' '] as &[_]);
    if result.is_empty() || result == pattern {
        format!("{safe_doi}.pdf")
    } else {
        result.to_string()
    }
}

fn sanitize(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '$'))
        .collect()
}

/// Lightweight metadata used for filename generation.
#[derive(Debug, Clone, Default)]
pub struct PaperMeta {
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<i32>,
    pub doi: Option<String>,
}
