use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, path::Path};

use finder::finder::PerceptualHash;
use finder::{
    finder::HammingClustererFinder,
    logger::{LogMsg, LoggerHandler, LoggerSender},
    state,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State};

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
    scan_state: Arc<AtomicU8>,
    logger_thread: Mutex<Option<JoinHandle<()>>>,
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

    let scan_state = state.scan_state.clone();
    scan_state.store(state::SCANNING, Ordering::Relaxed);

    let handle = tokio::task::spawn_blocking(move || {
        let mut clusterer = HammingClustererFinder::new(sender, scan_state.clone());
        clusterer.scan_directory(dir);
        let duplicates = clusterer.get_clustered_duplicates(5); // takes a long while.
        scan_state.store(state::IDLE, Ordering::Relaxed);

        duplicates
    });

    let res = handle
        .await
        .map_err(|e| e.to_string())
        .map(remap_hash_into_img);

    res
}

#[tauri::command]
async fn cancel(state: State<'_, AppState>, handle: AppHandle) -> Result<(), ()> {
    state.scan_state.store(state::CANCELLED, Ordering::Relaxed);
    let _ = handle.emit("process:cancelling", 0);

    while state.scan_state.load(Ordering::Relaxed) != state::IDLE {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let _ = handle.emit("process:finished", 0);

    Ok(())
}

fn logger(msg: &LogMsg, handle: &AppHandle) {
    let _ = match msg {
        LogMsg::Info(s) => handle.emit("log:info", s),
        LogMsg::Decoding(s) => {
            // println!("decoding.");
            handle.emit("file:decoding", s)
        }
        LogMsg::Hash(s) => {
            // println!("hashing.");
            handle.emit("file:hash", s)
        }
        LogMsg::Finished(s) => {
            // println!("finished.");
            handle.emit("file:finished", s)
        }
        LogMsg::ImageTotal(n) => handle.emit("file:total", n),
        LogMsg::FileError(file, err) => handle.emit("file:error", (file, err)),
        LogMsg::Error(err) => handle.emit("error", err),
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (handle, sender) = LoggerHandler::new();

    let mut shutting_down = false;

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let logger_thread =
                std::thread::spawn(|| handle.blocking_run(move |msg| logger(msg, &app_handle)));
            let appdata = AppState {
                sender: Mutex::new(Some(sender)),
                scan_state: Arc::new(AtomicU8::new(state::IDLE)),
                logger_thread: Mutex::new(Some(logger_thread)),
            };
            app.manage(appdata);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan, cancel, remove_file])
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
                println!("sent out signal to shut down.");
                state.scan_state.store(state::CANCELLED, Ordering::Relaxed);

                println!("waiting for threads to shut down.");
                while state.scan_state.load(Ordering::Relaxed) != state::IDLE {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }

                println!("dropping live communications.");
                drop(state.sender.lock().unwrap().take());
                if let Some(thread) = state.logger_thread.lock().unwrap().take() {
                    thread.join().expect("logger thread panicked.");
                }

                println!("exiting...");
                handle.exit(0);
            }
            _ => {}
        });
}
