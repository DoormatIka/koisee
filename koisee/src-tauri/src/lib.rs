use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, path::Path};

use finder::finder::PerceptualHash;
use finder::{
    finder::HammingClustererFinder,
    logger::{LogMsg, LoggerHandler, LoggerSender},
};
use serde::{Deserialize, Serialize};
use tauri::{Manager, RunEvent, State};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImageData {
    path: String,
    dimensions: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchItem {
    items: Vec<ImageData>,
    average_similarity: f32,
}

struct AppState {
    sender: Mutex<Option<LoggerSender>>,
    cancelled: Arc<AtomicBool>,
}

#[tauri::command]
async fn remove_file(path: &str) -> Result<(), String> {
    Ok(fs::remove_file(path).map_err(|e| e.to_string())?)
}

fn remap_hash_into_img(e: Vec<Vec<PerceptualHash>>) -> Vec<Vec<ImageData>> {
    let matches: Vec<Vec<ImageData>> = e
        .into_iter()
        .map(|t| {
            t.into_iter()
                .map(|v| ImageData {
                    path: v.path,
                    dimensions: v.dimensions,
                })
                .collect()
        })
        .collect();

    matches
}

#[tauri::command]
async fn scan(state: State<'_, AppState>, dir: String) -> Result<Vec<Vec<ImageData>>, String> {
    print!("{}", dir);
    let input_path = Path::new(&dir);
    if !input_path.is_dir() {
        return Err(String::from(
            "File provided is not a directory! Please choose another one.",
        ));
    }

    let sender = state.sender.lock().unwrap().clone();
    let Some(sender) = sender else {
        return Err("App is shutting down.".into());
    };

    let cancelled = state.cancelled.clone();

    let handle = tokio::task::spawn_blocking(move || {
        let mut clusterer = HammingClustererFinder::new(sender, cancelled);
        clusterer.scan_directory(dir);

        clusterer.get_clustered_duplicates(5) // takes a long while.
    });

    let res = handle
        .await
        .map_err(|e| e.to_string())
        .map(remap_hash_into_img);

    res
}

fn logger(msg: &LogMsg) {
    match msg {
        LogMsg::Info(s) => println!("[INFO]: {}", s),
        LogMsg::Hash(s) => println!("[HASHING] \"{}\"", s),
        LogMsg::Decoding(s) => println!("[DECODING] \"{}\"", s),
        LogMsg::Finished(s) => println!("   [FINISHED] \"{}\"", s),
        LogMsg::ImageTotal(n) => println!("[TOTAL] \"{}\"", n),
        LogMsg::Error(err) => println!("[ERR] {}", err),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (handle, sender) = LoggerHandler::new();
    let logger_thread = std::thread::spawn(|| handle.blocking_run(logger));

    let mut logger_thread = Some(logger_thread);
    let mut shutting_down = false;

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let appdata = AppState {
                sender: Mutex::new(Some(sender.clone())),
                cancelled: Arc::new(AtomicBool::new(false)),
            };
            app.manage(appdata);
            drop(sender);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan, remove_file])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                if shutting_down {
                    return;
                }
                shutting_down = true;

                api.prevent_exit();

                println!("prevented exit.");
                let state: State<AppState> = handle.state();
                state.cancelled.store(true, Ordering::Relaxed);

                drop(state.sender.lock().unwrap().take());

                if let Some(thread) = logger_thread.take() {
                    thread.join().expect("logger thread panicked.");
                }

                println!("exiting...");
                handle.exit(0);
            }
            _ => {}
        });
}
