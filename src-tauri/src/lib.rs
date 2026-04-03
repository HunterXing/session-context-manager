pub mod commands;
pub mod models;

use commands::{exporter, parser, scanner, search};
use models::Session;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePath {
    pub name: String,
    pub path: String,
    pub source_type: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub custom_paths: Vec<SourcePath>,
    pub export_path: String,
}

#[tauri::command]
fn scan() -> Result<Vec<Session>, String> {
    scanner::scan_sources()
}

#[tauri::command]
fn scan_with_paths(paths: Vec<SourcePath>) -> Result<Vec<Session>, String> {
    scanner::scan_with_custom_paths(&paths)
}

#[tauri::command]
fn get_session(path: String) -> Result<Session, String> {
    let path = Path::new(&path);
    if path.extension().map(|e| e == "jsonl").unwrap_or(false) {
        parser::parse_claude_jsonl(path)
    } else {
        parser::parse_opencode_json(path)
    }
}

#[tauri::command]
fn search(query: String, sessions: Vec<Session>) -> Vec<Session> {
    let results = search::search_sessions(&sessions, &query);
    results.into_iter().cloned().collect()
}

#[tauri::command]
fn export_session(session: Session, output_dir: String) -> Result<String, String> {
    let output_path = Path::new(&output_dir);
    exporter::export_to_markdown(&session, output_path)
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn export_project(
    project: String,
    sessions: Vec<Session>,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let output_path = Path::new(&output_dir);
    exporter::export_project(&project, &sessions, output_path)
        .map(|paths| paths.into_iter().map(|p| p.to_string_lossy().to_string()).collect())
}

#[tauri::command]
fn get_default_source_paths() -> Vec<(String, String)> {
    scanner::get_default_paths()
}

#[tauri::command]
async fn save_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    let store = app.store("config.json").map_err(|e| e.to_string())?;
    store.set("config", serde_json::to_value(&config).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn load_config(app: tauri::AppHandle) -> Result<AppConfig, String> {
    let store = app.store("config.json").map_err(|e| e.to_string())?;
    if let Some(value) = store.get("config") {
        Ok(serde_json::from_value(value.clone()).map_err(|e| e.to_string())?)
    } else {
        Ok(AppConfig::default())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            scan,
            scan_with_paths,
            get_session,
            search,
            export_session,
            export_project,
            get_default_source_paths,
            save_config,
            load_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
