use std::{fs, path::Path};

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
    sender: LoggerSender,
}

#[tauri::command]
async fn remove_file(path: &str) -> Result<(), String> {
    Ok(fs::remove_file(path).map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn scan(state: State<'_, AppState>, dir: String) -> Result<Vec<Vec<ImageData>>, String> {
    let input_path = Path::new(&dir);
    if !input_path.is_dir() {
        return Err(String::from(
            "File provided is not a directory! Please choose another one.",
        ));
    }

    let mut clusterer = HammingClustererFinder::new(state.sender.clone());
    clusterer.scan_directory(dir);
    let best_matches = clusterer.get_clustered_duplicates(5);
    let matches: Vec<Vec<ImageData>> = best_matches
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

    Ok(matches)
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

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let appdata = AppState {
                sender: sender.clone(),
            };
            app.manage(appdata);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan, remove_file])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_, event| match event {
            RunEvent::ExitRequested { .. } => {
                if let Some(thread) = logger_thread.take() {
                    thread.join().expect("logger thread panicked.");
                }
            }
            _ => {}
        });
}
