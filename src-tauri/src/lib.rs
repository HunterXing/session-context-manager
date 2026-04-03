pub mod commands;
pub mod models;

use commands::{exporter, parser, scanner, search};
use models::Session;
use std::path::Path;

#[tauri::command]
fn scan() -> Result<Vec<Session>, String> {
    scanner::scan_sources()
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan,
            get_session,
            search,
            export_session,
            export_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
