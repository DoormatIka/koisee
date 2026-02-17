// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri::{AppHandle, Manager, State};
use tokio::time;

#[derive(Debug, Serialize, Deserialize)]
struct ImageItem {
    paths: [String; 2],
    similarity: f32,
}
#[derive(Debug, Serialize, Deserialize)]
struct ScanResult {
    matched_images: Vec<ImageItem>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ScanInput {
    dir: String,
}

struct AppState {
    client: reqwest::Client,
    is_server_alive: AtomicBool,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_heartbeat(state: State<'_, AppState>) -> Result<bool, ()> {
    Ok(state.is_server_alive.load(Ordering::Relaxed))
}

#[tauri::command]
async fn get_similar_images(state: State<'_, AppState>, dir: String) -> Result<ScanResult, String> {
    let is_server_alive = state.is_server_alive.load(Ordering::Relaxed);
    if !is_server_alive {
        return Err(String::from("Server is not online!"));
    }
    let input_path = Path::new(&dir);
    if !input_path.is_dir() {
        return Err(String::from(
            "File provided is not a directory! Please choose another one.",
        ));
    }

    let client = &state.client;
    let res = client
        .post("http://localhost:8080/scan")
        .json(&ScanInput { dir: dir })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let res = res.json::<ScanResult>().await.map_err(|e| e.to_string())?;

    Ok(res)
}

fn setup_heartbeat(handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            let state = handle.state::<AppState>();

            let is_alive = client
                .get("http://localhost:8080/heartbeat")
                .timeout(Duration::from_secs(2))
                .send()
                .await
                .is_ok();
            state.is_server_alive.store(is_alive, Ordering::Relaxed);

            handle.emit("server-status", is_alive).unwrap();
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let appdata = AppState {
                client: reqwest::Client::new(),
                is_server_alive: AtomicBool::new(false),
            };
            app.manage(appdata);

            setup_heartbeat(app.handle().clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_heartbeat,
            get_similar_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
