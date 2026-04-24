use std::{
    fs, path::{Path, PathBuf}, sync::{
        Mutex, atomic::{AtomicBool, Ordering}
    }, time::Duration
};

use serde::{Deserialize, Serialize};
use tauri::{Emitter, RunEvent};
use tauri::{AppHandle, Manager, Runtime, State};

#[derive(Debug, Serialize, Deserialize)]
struct ScanInput {
    dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImageData {
  path: String,
  width: i32,
  height: i32,
  similarity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageItem {
    uuid: String,
    paths: Vec<ImageData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: String,
    directory: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "status")]
pub enum ScanIntermediateResult {
    #[serde(rename = "result")]
    Result { matched_images: Vec<ImageItem> },
    #[serde(rename = "error")]
    Error { error: String },
    #[serde(rename = "progress")]
    InProgress,
    #[serde(rename = "none")]
    NoneFound,
}

struct AppState {
    queued: Mutex<Vec<String>>,
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_heartbeat(state: State<'_, AppState>) -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
async fn remove_file(path: &str) -> Result<(), String> {
    Ok(fs::remove_file(path).map_err(|e| e.to_string())?)
}


#[tauri::command]
async fn queue_scan(state: State<'_, AppState>, dir: String) -> Result<String, String> {
    let input_path = Path::new(&dir);
    if !input_path.is_dir() {
        return Err(String::from(
            "File provided is not a directory! Please choose another one.",
        ));
    }

    Err(String::from("In progress."))
}

#[tauri::command]
async fn get_scan_result(state: State<'_, AppState>, uuid: String) -> Result<ScanIntermediateResult, String> {
    Ok(ScanIntermediateResult::NoneFound)
}

fn remove_quotes(s: &str) -> &str {
    s.strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(s)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let appdata = AppState {
                queued: Mutex::new(Vec::new()),
            };
            app.manage(appdata);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_heartbeat,
            queue_scan,
            get_scan_result,
            remove_file,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|handle, event| match event {
            RunEvent::Exit => {
                let appdata = handle.state::<AppState>();
                println!("killing child.");
            },
            _ => {}
        });
}
