use std::{
    path::{Path, PathBuf}, sync::{
        Mutex, atomic::{AtomicBool, Ordering}
    }, time::Duration
};

use serde::{Deserialize, Serialize};
use tauri::{Emitter, RunEvent};
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_shell::{ShellExt, process::{CommandEvent, CommandChild}};
use tokio::time;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageItem {
    paths: [String; 2],
    similarity: f32,
}
#[derive(Debug, Serialize, Deserialize)]
struct ScanInput {
    dir: String,
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
    client: reqwest::Client,
    is_server_alive: AtomicBool,
    queued: Mutex<Vec<String>>,
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
async fn queue_scan(state: State<'_, AppState>, dir: String) -> Result<String, String> {
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
    let res = client.post("http://localhost:8080/scan")
        .json(&ScanInput { dir: dir })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let res = res.text().await.map_err(|e| e.to_string())?;

    {
        let mut queued = state.queued.lock().unwrap();
        queued.push(res.clone());
    }

    Ok(res)
}

#[tauri::command]
async fn get_scan_result(state: State<'_, AppState>, uuid: String) -> Result<ScanIntermediateResult, String> {
    let is_server_alive = state.is_server_alive.load(Ordering::Relaxed);
    if !is_server_alive {
        return Err(String::from("Server is not online!"));
    }
    let client = &state.client;
    let queue = client.get(format!("http://localhost:8080/scan/{uuid}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let queue: ScanIntermediateResult = queue.json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(queue)
}

fn setup_heartbeat(handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let mut interval = time::interval(Duration::from_secs(3));

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

fn get_classifier_path<R: Runtime>(handle: &AppHandle<R>) -> PathBuf {
    // OS picking
    let target = env!("APP_TARGET");
    let binary_name = if cfg!(windows) {
        format!("classifier-{}.exe", target)
    } else {
        format!("classifier-{}", target)
    };

    // standard path
    let installed_path = handle.path().resource_dir()
        .expect("Failed to resolve resource directory")
        .join("binaries")
        .join("classifier-bundle")
        .join(&binary_name);

    // portable path
    if !installed_path.exists() {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let app_dir = exe_path.parent().expect("Failed to get parent dir");
        
        return app_dir
            .join("binaries")
            .join("classifier-bundle")
            .join(&binary_name);
    }

    installed_path
}


fn setup_classifier(handle: AppHandle) {
    let exec_path = get_classifier_path(&handle);
    let lib_exec_path = exec_path.clone();
    let lib_path = lib_exec_path.parent().unwrap();
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

fn setup_job_emitter(handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let mut interval = time::interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            let state = handle.state::<AppState>();

            let queued = {
                let queued = state.queued.lock().unwrap();
                queued.clone()
            };
            for uuid in queued {
                let intermediate_result = client
                    .get(format!("http://localhost:8080/scan/{uuid}"))
                    .timeout(Duration::from_secs(2))
                    .send()
                    .await
                    .unwrap();
                let intermediate_result = intermediate_result
                    .json::<ScanIntermediateResult>()
                    .await
                    .unwrap();
                match intermediate_result {
                    ScanIntermediateResult::Result { matched_images } => {
                        handle.emit("scan-finished", matched_images).unwrap();
                    },
                    ScanIntermediateResult::Error { error } => {
                        handle.emit("scan-error", error).unwrap();
                    },
                    ScanIntermediateResult::InProgress
                    | ScanIntermediateResult::NoneFound => { 
                        println!("inprogress, nonefound");
                    }
                }
            }

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
                queued: Mutex::new(Vec::new()),
                child: Mutex::new(None)
            };
            app.manage(appdata);

            setup_heartbeat(app.handle().clone());
            setup_classifier(app.handle().clone());
            setup_job_emitter(app.handle().clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_heartbeat,
            queue_scan,
            get_scan_result,
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
