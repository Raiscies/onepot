// use std::fs

use crate::config::get;
use crate::config::set;
use crate::citation_parse::{parse_single, set_runner_path, split_citations};
use crate::paper::{ParseResult, PaperStatus};
use crate::CitationTextWrapper;
use crate::CitationResultsWrapper;
use crate::StringWrapper;
use crate::APP;
use log::{info, warn};
use tauri::Manager;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tauri::Monitor;
use tauri::Window;
use tauri::WindowBuilder;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;

// Get daemon window instance
fn get_daemon_window() -> Window {
    let app_handle = APP.get().unwrap();
    match app_handle.get_window("daemon") {
        Some(v) => v,
        None => {
            warn!("Daemon window not found, create new daemon window!");
            WindowBuilder::new(
                app_handle,
                "daemon",
                tauri::WindowUrl::App("daemon.html".into()),
            )
            .title("Daemon")
            .additional_browser_args("--disable-web-security")
            .visible(false)
            .build()
            .unwrap()
        }
    }
}

// Get monitor where the mouse is currently located
fn get_current_monitor(x: i32, y: i32) -> Monitor {
    info!("Mouse position: {}, {}", x, y);
    let daemon_window = get_daemon_window();
    let monitors = daemon_window.available_monitors().unwrap();

    for m in monitors {
        let size = m.size();
        let position = m.position();

        if x >= position.x
            && x <= (position.x + size.width as i32)
            && y >= position.y
            && y <= (position.y + size.height as i32)
        {
            info!("Current Monitor: {:?}", m);
            return m;
        }
    }
    warn!("Current Monitor not found, using primary monitor");
    daemon_window.primary_monitor().unwrap().unwrap()
}

// Creating a window on the mouse monitor
fn build_window(label: &str, title: &str) -> (Window, bool) {
    use mouse_position::mouse_position::{Mouse, Position};

    let mouse_position = match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Position { x, y },
        Mouse::Error => {
            warn!("Mouse position not found, using (0, 0) as default");
            Position { x: 0, y: 0 }
        }
    };
    let current_monitor = get_current_monitor(mouse_position.x, mouse_position.y);
    let position = current_monitor.position();

    let app_handle = APP.get().unwrap();
    match app_handle.get_window(label) {
        Some(v) => {
            info!("Window existence: {}", label);
            v.set_focus().unwrap();
            (v, true)
        }
        None => {
            info!("Window not existence, Creating new window: {}", label);
            let mut builder = tauri::WindowBuilder::new(
                app_handle,
                label,
                tauri::WindowUrl::App("index.html".into()),
            )
            .position(position.x.into(), position.y.into())
            .additional_browser_args("--disable-web-security")
            .focused(true)
            .title(title)
            .visible(false);

            #[cfg(target_os = "macos")]
            {
                builder = builder
                    .title_bar_style(tauri::TitleBarStyle::Overlay)
                    .hidden_title(true);
            }
            #[cfg(not(target_os = "macos"))]
            {
                builder = builder.transparent(true).decorations(false);
            }
            let window = builder.build().unwrap();

            if label != "screenshot" {
                #[cfg(not(target_os = "linux"))]
                set_shadow(&window, true).unwrap_or_default();
            }
            let _ = window.current_monitor();
            (window, false)
        }
    }
}

pub fn config_window() {
    let (window, _exists) = build_window("config", "Config");
    window
        .set_min_size(Some(tauri::LogicalSize::new(800, 400)))
        .unwrap();
    window.set_size(tauri::LogicalSize::new(800, 600)).unwrap();
    window.center().unwrap();
}

fn translate_window() -> Window {
    use mouse_position::mouse_position::{Mouse, Position};
    // Mouse physical position
    let mut mouse_position = match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Position { x, y },
        Mouse::Error => {
            warn!("Mouse position not found, using (0, 0) as default");
            Position { x: 0, y: 0 }
        }
    };
    let (window, exists) = build_window("translate", "Translate");
    if exists {
        return window;
    }
    window.set_skip_taskbar(true).unwrap();
    // Get Translate Window Size
    let width = match get("translate_window_width") {
        Some(v) => v.as_i64().unwrap(),
        None => {
            set("translate_window_width", 350);
            350
        }
    };
    let height = match get("translate_window_height") {
        Some(v) => v.as_i64().unwrap(),
        None => {
            set("translate_window_height", 420);
            420
        }
    };

    let monitor = window.current_monitor().unwrap().unwrap();
    let dpi = monitor.scale_factor();

    window
        .set_size(tauri::PhysicalSize::new(
            (width as f64) * dpi,
            (height as f64) * dpi,
        ))
        .unwrap();

    let position_type = match get("translate_window_position") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => "mouse".to_string(),
    };

    match position_type.as_str() {
        "mouse" => {
            // Adjust window position
            let monitor_size = monitor.size();
            let monitor_size_width = monitor_size.width as f64;
            let monitor_size_height = monitor_size.height as f64;
            let monitor_position = monitor.position();
            let monitor_position_x = monitor_position.x as f64;
            let monitor_position_y = monitor_position.y as f64;

            if mouse_position.x as f64 + width as f64 * dpi
                > monitor_position_x + monitor_size_width
            {
                mouse_position.x -= (width as f64 * dpi) as i32;
                if (mouse_position.x as f64) < monitor_position_x {
                    mouse_position.x = monitor_position_x as i32;
                }
            }
            if mouse_position.y as f64 + height as f64 * dpi
                > monitor_position_y + monitor_size_height
            {
                mouse_position.y -= (height as f64 * dpi) as i32;
                if (mouse_position.y as f64) < monitor_position_y {
                    mouse_position.y = monitor_position_y as i32;
                }
            }

            window
                .set_position(tauri::PhysicalPosition::new(
                    mouse_position.x,
                    mouse_position.y,
                ))
                .unwrap();
        }
        _ => {
            let position_x = match get("translate_window_position_x") {
                Some(v) => v.as_i64().unwrap(),
                None => 0,
            };
            let position_y = match get("translate_window_position_y") {
                Some(v) => v.as_i64().unwrap(),
                None => 0,
            };
            window
                .set_position(tauri::PhysicalPosition::new(
                    (position_x as f64) * dpi,
                    (position_y as f64) * dpi,
                ))
                .unwrap();
        }
    }

    window
}

pub fn selection_translate() {
    use selection::get_text;
    // Get Selected Text
    let text = get_text();
    if !text.trim().is_empty() {
        let app_handle = APP.get().unwrap();
        // Write into State
        let state: tauri::State<StringWrapper> = app_handle.state();
        state.0.lock().unwrap().replace_range(.., &text);
    }

    let window = translate_window();
    window.emit("new_text", text).unwrap();
}

pub fn input_translate() {
    let app_handle = APP.get().unwrap();
    // Clear State
    let state: tauri::State<StringWrapper> = app_handle.state();
    state
        .0
        .lock()
        .unwrap()
        .replace_range(.., "[INPUT_TRANSLATE]");
    let window = translate_window();
    let position_type = match get("translate_window_position") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => "mouse".to_string(),
    };
    if position_type == "mouse" {
        window.center().unwrap();
    }

    window.emit("new_text", "[INPUT_TRANSLATE]").unwrap();
}

pub fn text_translate(text: String) {
    let app_handle = APP.get().unwrap();
    // Clear State
    let state: tauri::State<StringWrapper> = app_handle.state();
    state.0.lock().unwrap().replace_range(.., &text);
    let window = translate_window();
    window.emit("new_text", text).unwrap();
}

pub fn image_translate() {
    let app_handle = APP.get().unwrap();
    let state: tauri::State<StringWrapper> = app_handle.state();
    state
        .0
        .lock()
        .unwrap()
        .replace_range(.., "[IMAGE_TRANSLATE]");
    let window = translate_window();
    window.emit("new_text", "[IMAGE_TRANSLATE]").unwrap();
}

pub fn recognize_window() {
    let (window, exists) = build_window("recognize", "Recognize");
    if exists {
        window.emit("new_image", "").unwrap();
        return;
    }
    let width = match get("recognize_window_width") {
        Some(v) => v.as_i64().unwrap(),
        None => {
            set("recognize_window_width", 800);
            800
        }
    };
    let height = match get("recognize_window_height") {
        Some(v) => v.as_i64().unwrap(),
        None => {
            set("recognize_window_height", 400);
            400
        }
    };
    let monitor = window.current_monitor().unwrap().unwrap();
    let dpi = monitor.scale_factor();
    window
        .set_size(tauri::PhysicalSize::new(
            (width as f64) * dpi,
            (height as f64) * dpi,
        ))
        .unwrap();
    window.center().unwrap();
    window.emit("new_image", "").unwrap();
}

#[cfg(not(target_os = "macos"))]
fn screenshot_window() -> Window {
    let (window, _exists) = build_window("screenshot", "Screenshot");

    window.set_skip_taskbar(true).unwrap();
    #[cfg(target_os = "macos")]
    {
        let monitor = window.current_monitor().unwrap().unwrap();
        let size = monitor.size();
        window.set_decorations(false).unwrap();
        window.set_size(*size).unwrap();
    }

    #[cfg(not(target_os = "macos"))]
    window.set_fullscreen(true).unwrap();

    window.set_always_on_top(true).unwrap();
    window
}

pub fn ocr_recognize() {
    #[cfg(target_os = "macos")]
    {
        let app_handle = APP.get().unwrap();
        let mut app_cache_dir_path = cache_dir().expect("Get Cache Dir Failed");
        app_cache_dir_path.push(&app_handle.config().tauri.bundle.identifier);
        if !app_cache_dir_path.exists() {
            // 创建目录
            fs::create_dir_all(&app_cache_dir_path).expect("Create Cache Dir Failed");
        }
        app_cache_dir_path.push("pot_screenshot_cut.png");

        let path = app_cache_dir_path.to_string_lossy().replace("\\\\?\\", "");
        println!("Screenshot path: {}", path);
        if let Ok(_output) = std::process::Command::new("/usr/sbin/screencapture")
            .arg("-i")
            .arg("-r")
            .arg(path)
            .output()
        {
            recognize_window();
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let window = screenshot_window();
        let window_ = window.clone();
        window.listen("success", move |event| {
            recognize_window();
            window_.unlisten(event.id())
        });
    }
}
pub fn ocr_translate() {
    #[cfg(target_os = "macos")]
    {
        let app_handle = APP.get().unwrap();
        let mut app_cache_dir_path = cache_dir().expect("Get Cache Dir Failed");
        app_cache_dir_path.push(&app_handle.config().tauri.bundle.identifier);
        if !app_cache_dir_path.exists() {
            // 创建目录
            fs::create_dir_all(&app_cache_dir_path).expect("Create Cache Dir Failed");
        }
        app_cache_dir_path.push("pot_screenshot_cut.png");

        let path = app_cache_dir_path.to_string_lossy().replace("\\\\?\\", "");
        println!("Screenshot path: {}", path);
        if let Ok(_output) = std::process::Command::new("/usr/sbin/screencapture")
            .arg("-i")
            .arg("-r")
            .arg(path)
            .output()
        {
            image_translate();
            ();
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let window = screenshot_window();
        let window_ = window.clone();
        window.listen("success", move |event| {
            image_translate();
            window_.unlisten(event.id())
        });
    }
}

#[tauri::command(async)]
pub fn updater_window() {
    let (window, _exists) = build_window("updater", "Updater");
    window
        .set_min_size(Some(tauri::LogicalSize::new(600, 400)))
        .unwrap();
    window.set_size(tauri::LogicalSize::new(600, 400)).unwrap();
    window.center().unwrap();
}

/// Incremented on each search to cancel stale tasks.
static CITATION_SEARCH_ID: AtomicU64 = AtomicU64::new(0);

/// Hotkey handler: capture selected text, split citations, push placeholders,
/// then spawn parallel AnyStyle parse tasks.
pub fn citation_selection() {
    use selection::get_text;

    let text = get_text();
    if text.trim().is_empty() {
        return;
    }

    let app_handle = APP.get().unwrap();
    let state: tauri::State<CitationTextWrapper> = app_handle.state();
    state.0.lock().unwrap().replace_range(.., &text);

    // Init runner path relative to src-tauri (Cargo workspace root)
    let _ = set_runner_path("resources/anystyle/runner.rb");

    // Cancel previous search
    let my_id = CITATION_SEARCH_ID.fetch_add(1, Ordering::SeqCst) + 1;
    info!("citation_search #{my_id}: starting");

    // Build or reuse window (invisible, frontend shows itself)
    let (window, exists) = build_window("citation", "Citation");
    if exists {
        // Restore remembered position if configured
        let position_type = get("citation_window_position")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or("mouse".to_string());
        if position_type != "mouse" {
            let dpi = window.current_monitor().unwrap().unwrap().scale_factor();
            let pos_x = get("citation_window_position_x").and_then(|v| v.as_i64()).unwrap_or(0);
            let pos_y = get("citation_window_position_y").and_then(|v| v.as_i64()).unwrap_or(0);
            window.set_position(tauri::PhysicalPosition::new((pos_x as f64) * dpi, (pos_y as f64) * dpi)).unwrap();
        }
    } else {
        // New window setup
        let always_on_top = get("citation_always_on_top")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        window.set_always_on_top(always_on_top).unwrap();
        window.set_min_size(Some(tauri::LogicalSize::new(300.0, 200.0))).unwrap();
        window.set_size(tauri::LogicalSize::new(400.0, 500.0)).unwrap();

        // Position: mouse mode is already handled by build_window; pre_state restores saved pos
        let position_type = get("citation_window_position")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or("mouse".to_string());
        if position_type != "mouse" {
            let dpi = window.current_monitor().unwrap().unwrap().scale_factor();
            let pos_x = get("citation_window_position_x").and_then(|v| v.as_i64()).unwrap_or(0);
            let pos_y = get("citation_window_position_y").and_then(|v| v.as_i64()).unwrap_or(0);
            window.set_position(tauri::PhysicalPosition::new((pos_x as f64) * dpi, (pos_y as f64) * dpi)).unwrap();
        }
    }

    // Split and emit initial placeholders (with raw citation text)
    let segments = split_citations(&text);
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

    // Store initial placeholders in shared state
    {
        let results: tauri::State<CitationResultsWrapper> = app_handle.state();
        *results.0.lock().unwrap() = placeholders.clone();
    }

    let payload = serde_json::json!({
        "papers": placeholders,
        "captured_text": text,
        "total": total,
    }).to_string();
    let _ = window.emit("citation_init", payload);
    info!("citation_search #{my_id}: emitted init, {total} papers");

    // Spawn parallel AnyStyle parse tasks
    for (i, (citation_index, citation_text)) in segments.into_iter().enumerate() {
        let w = window.clone();
        let app = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            if CITATION_SEARCH_ID.load(Ordering::SeqCst) != my_id {
                return;
            }

            let mut result = parse_single(&citation_text, i, citation_index.as_deref());

            if CITATION_SEARCH_ID.load(Ordering::SeqCst) != my_id {
                return;
            }

            result.paper.status = PaperStatus::Ready;

            // Write into shared results so frontend can pull after mount
            {
                let results: tauri::State<CitationResultsWrapper> = app.state();
                let mut guard = results.0.lock().unwrap();
                if i < guard.len() {
                    guard[i] = result.clone();
                }
            }

            let update = serde_json::json!({
                "index": i,
                "phase": "parsed",
                "data": result,
            });
            let _ = w.emit("citation_update", update.to_string());
            info!("citation_search #{my_id}: paper #{i} parsed");
        });
    }
}

#[tauri::command]
pub fn get_citation_state() -> String {
    let app_handle = APP.get().unwrap();
    let text_state: tauri::State<CitationTextWrapper> = app_handle.state();
    let text = text_state.0.lock().unwrap().clone();
    if text.is_empty() {
        return "{}".to_string();
    }
    let results_state: tauri::State<CitationResultsWrapper> = app_handle.state();
    let papers = results_state.0.lock().unwrap().clone();
    serde_json::json!({
        "papers": papers,
        "captured_text": text,
        "total": papers.len(),
    }).to_string()
}

#[tauri::command(async)]
pub fn open_citation_window() {
    let (window, _exists) = build_window("citation", "Citation");
    window
        .set_min_size(Some(tauri::LogicalSize::new(320, 200)))
        .unwrap();
    window.set_size(tauri::LogicalSize::new(400, 500)).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}
