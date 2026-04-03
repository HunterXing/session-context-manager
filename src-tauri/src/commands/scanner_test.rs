use crate::commands::scanner;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_scanner_returns_result() {
    let result = scanner::scan_sources();
    assert!(result.is_ok());
}

#[test]
fn test_scanner_handles_missing_claude_dir() {
    let result = scanner::scan_claude_sessions();
    assert!(result.is_ok());
}

#[test]
fn test_scanner_handles_missing_opencode_dir() {
    let result = scanner::scan_opencode_sessions();
    assert!(result.is_ok());
}

#[test]
fn test_scanner_with_temp_claude_dir() {
    let dir = tempdir().unwrap();
    let sessions_dir = dir.path().join("sessions");
    fs::create_dir_all(&sessions_dir).unwrap();

    let test_file = sessions_dir.join("test_session.jsonl");
    File::create(&test_file).unwrap().write_all(b"test content\n").unwrap();

    let result = scanner::scan_sources();
    assert!(result.is_ok());
}
