//! Custom download handler registry.
//!
//! Each submodule in `download_handlers/` exposes a `pub(super) fn handler`
//! matching the `CustomHandlerFn` signature.
//! The JSON `"handler": "elsevier"` maps to `download_handlers/elsevier.rs`.
//!
//! To add a new custom handler:
//! 1. Create `download_handlers/<name>.rs` with `pub(super) fn handler`
//! 2. Add `mod <name>;` below
//! 3. Add `m.insert("<name>", <name>::handler as CustomHandlerFn);` below

use crate::download::CustomHandlerFn;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) mod default;
mod elsevier;

pub static HANDLER_REGISTRY: Lazy<HashMap<&'static str, CustomHandlerFn>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("elsevier", elsevier::handler as CustomHandlerFn);
    m
});

/// Look up a custom handler by its JSON config name. Returns `None` for unknown names.
pub(crate) fn lookup(name: &str) -> Option<CustomHandlerFn> {
    HANDLER_REGISTRY.get(name).copied()
}
