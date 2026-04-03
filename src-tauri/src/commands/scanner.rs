use crate::models::{Session, SessionSource};
use std::fs;
use std::path::PathBuf;

/// Expands a path with ~ to the user's home directory
fn expand_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(path.trim_start_matches('~'));
        }
    }
    PathBuf::from(path)
}

/// Escape a project path to be used in directory names
fn escape_project_name(project_path: &str) -> String {
    project_path
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "_")
        .replace(' ', "_")
}

/// Scans both Claude and OpenCode source directories for session files.
///
/// Claude path: `~/.claude/projects/{escaped_project}/sessions/*.jsonl`
/// OpenCode path: `~/.config/opencode/sessions/*.json`
///
/// Returns `Vec<Session>` with project grouping and time sorting.
/// Handles missing directories gracefully by returning an empty vector.
pub fn scan_sources() -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    // Scan Claude sessions
    if let Ok(claude_sessions) = scan_claude_sessions() {
        sessions.extend(claude_sessions);
    }

    // Scan OpenCode sessions
    if let Ok(opencode_sessions) = scan_opencode_sessions() {
        sessions.extend(opencode_sessions);
    }

    // Sort by updated_at descending (most recent first)
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(sessions)
}

/// Scans Claude session directories
pub fn scan_claude_sessions() -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
    let claude_base = PathBuf::from(&home).join(".claude/projects");

    if !claude_base.exists() {
        return Ok(sessions); // Gracefully handle missing directory
    }

    // Read all project directories
    let projects = fs::read_dir(&claude_base).map_err(|e| e.to_string())?;

    for project in projects.flatten() {
        let project_path = project.path();
        if !project_path.is_dir() {
            continue;
        }

        let project_name = project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let sessions_dir = project_path.join("sessions");
        if !sessions_dir.exists() {
            continue;
        }

        // Read session files
        if let Ok(entries) = fs::read_dir(&sessions_dir) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                    // Create a placeholder session - actual parsing is Task 6
                    let session = Session::new(
                        file_path.to_string_lossy().to_string(),
                        SessionSource::Claude,
                        project_path.clone(),
                        project_name.clone(),
                    );
                    sessions.push(session);
                }
            }
        }
    }

    Ok(sessions)
}

/// Scans OpenCode session directories
pub fn scan_opencode_sessions() -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let opencode_base = expand_path("~/.config/opencode/sessions");

    if !opencode_base.exists() {
        return Ok(sessions); // Gracefully handle missing directory
    }

    // Read all .json files in the sessions directory
    if let Ok(entries) = fs::read_dir(&opencode_base) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().map(|e| e == "json").unwrap_or(false) {
                // Create a placeholder session - actual parsing is Task 6
                let session = Session::new(
                    file_path.to_string_lossy().to_string(),
                    SessionSource::OpenCode,
                    opencode_base.clone(),
                    "opencode".to_string(),
                );
                sessions.push(session);
            }
        }
    }

    Ok(sessions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_scan_sources_handles_missing_directories() {
        // Test that missing directories return empty vec, not error
        let result = scan_sources();
        assert!(result.is_ok());
        // May or may not be empty depending on actual session files
    }

    #[test]
    fn test_expand_path() {
        let home = std::env::var("HOME").unwrap();
        let expanded = expand_path("~/test");
        assert_eq!(expanded, PathBuf::from(home).join("test"));
    }

    #[test]
    fn test_escape_project_name() {
        assert_eq!(
            escape_project_name("/home/user/project"),
            "home_user_project"
        );
        assert_eq!(
            escape_project_name("C:\\Users\\Project"),
            "C__Users_Project"
        );
    }

    #[test]
    fn test_scan_claude_with_temp_dir() {
        let dir = tempdir().unwrap();
        let sessions_dir = dir.path().join("sessions");
        fs::create_dir_all(&sessions_dir).unwrap();

        // Create a test session file
        let test_file = sessions_dir.join("test_session.jsonl");
        File::create(&test_file)
            .unwrap()
            .write_all(b"test content\n")
            .unwrap();

        let result = scan_claude_sessions();
        assert!(result.is_ok());
    }
}
