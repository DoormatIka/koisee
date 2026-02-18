// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


use std::{
    path::Path, sync::{
        Mutex, atomic::{AtomicBool, Ordering}
    }, time::Duration
};

use serde::{Deserialize, Serialize};
use tauri::{Emitter, RunEvent};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::{ShellExt, process::{CommandEvent, CommandChild}};
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
    child: Mutex<Option<CommandChild>>,
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
        let mut interval = time::interval(Duration::from_secs(5));

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
fn setup_classifier(handle: AppHandle) {
    let resource_dir = handle.path().resource_dir().expect("No resource directory found!");

    // need to handle windows builds as well.
    let lib_path = resource_dir.join("binaries/classifier-bundle");
    let target = env!("APP_TARGET");
    let cmd_name = format!("classifier-{}", target);
    let exec_path = lib_path.join(&cmd_name);

    let classifier_sidecar = handle.shell().command(exec_path)
        .current_dir(lib_path.to_string_lossy().to_string());

    let (mut rx, child) = classifier_sidecar
        .spawn()
        .expect("Failed to spawn sidecar.");

    {
        let guard_handle = handle.clone();
        let appdata = guard_handle.state::<AppState>();
        let mut guard = appdata.child.lock().unwrap();
        *guard = Some(child);
    }

    
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    print!("out: {}", line);
                    handle.emit("message", Some(format!("'{}'", line)))
                        .expect("failed to emit event");
                },
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    print!("err: {}", line);
                }
                _ => {}
            };
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let appdata = AppState {
                client: reqwest::Client::new(),
                is_server_alive: AtomicBool::new(false),
                child: Mutex::new(None)
            };
            app.manage(appdata);

            setup_heartbeat(app.handle().clone());
            setup_classifier(app.handle().clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_heartbeat,
            get_similar_images
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|handle, event| match event {
            RunEvent::Exit => {
                let appdata = handle.state::<AppState>();
                let mut state = appdata.child.lock().unwrap();
                if let Some(child) = state.take() {
                    let _ = child.kill().unwrap();
                    println!("killing child.");
                }
            },
            _ => {}
        });
}
