use crate::models::{Session, SessionSource};
use std::fs;
use std::path::PathBuf;

/// Represents the detected runtime environment
#[derive(Debug, Clone)]
pub enum RuntimeEnvironment {
    /// Native Linux/macOS with HOME set
    Native,
    /// Windows with WSL2 Ubuntu
    WindowsWsl2 {
        ubuntu_home: PathBuf,
        username: String,
    },
    /// Windows without WSL2 (native Windows paths)
    WindowsNative,
}

impl RuntimeEnvironment {
    /// Detect the current runtime environment
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            // Check if WSL2 Ubuntu is available via \\wsl$\Ubuntu
            let wsl_path = PathBuf::from(r"\\wsl$\Ubuntu");
            if wsl_path.exists() {
                // Try to find Ubuntu username from /etc/passwd via WSL
                // Common locations for Ubuntu home in WSL2
                if let Ok(entries) = fs::read_dir(&wsl_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            // Check for home directory
                            let home_dir = path.join("home");
                            if home_dir.exists() {
                                if let Ok(sub_entries) = fs::read_dir(&home_dir) {
                                    for sub in sub_entries.flatten() {
                                        let user_home = sub.path();
                                        if user_home.is_dir() {
                                            let username = user_home
                                                .file_name()
                                                .map(|n| n.to_string_lossy().to_string())
                                                .unwrap_or_default();
                                            return RuntimeEnvironment::WindowsWsl2 {
                                                ubuntu_home: user_home,
                                                username,
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Fallback: try to detect from WSL environment
                if let Ok(home) = std::env::var("WSL_HOME") {
                    if let Ok(username) = std::env::var("WSL_USERNAME") {
                        return RuntimeEnvironment::WindowsWsl2 {
                            ubuntu_home: PathBuf::from(&home),
                            username,
                        };
                    }
                }
            }
            return RuntimeEnvironment::WindowsNative;
        }

        #[cfg(not(target_os = "windows"))]
        {
            RuntimeEnvironment::Native
        }
    }

    /// Get the base home directory path for session scanning
    pub fn get_home(&self) -> Option<PathBuf> {
        match self {
            RuntimeEnvironment::Native => std::env::var("HOME").ok().map(PathBuf::from),
            RuntimeEnvironment::WindowsWsl2 { ubuntu_home, .. } => Some(ubuntu_home.clone()),
            RuntimeEnvironment::WindowsNative => std::env::var("USERPROFILE").ok().map(PathBuf::from),
        }
    }

    /// Get the path for Claude sessions
    pub fn get_claude_base(&self) -> Option<PathBuf> {
        self.get_home().map(|h| h.join(".claude/projects"))
    }

    /// Get the path for OpenCode sessions
    pub fn get_opencode_base(&self) -> Option<PathBuf> {
        match self {
            RuntimeEnvironment::Native | RuntimeEnvironment::WindowsWsl2 { .. } => {
                self.get_home().map(|h| h.join(".config/opencode/sessions"))
            }
            RuntimeEnvironment::WindowsNative => {
                self.get_home().map(|h| h.join(".config/opencode/sessions"))
            }
        }
    }
}

/// Expands a path with ~ to the user's home directory
fn expand_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(path.trim_start_matches('~'));
        }
        // Also check WSL_HOME for Windows
        if let Ok(wsldir) = std::env::var("WSL_HOME") {
            return PathBuf::from(wsldir).join(path.trim_start_matches('~'));
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
/// Returns `Vec<Session>` with project grouping and time sorting.
/// Handles missing directories gracefully by returning an empty vector.
pub fn scan_sources() -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let env = RuntimeEnvironment::detect();

    // Scan Claude sessions
    if let Ok(claude_sessions) = scan_claude_sessions_internal(&env) {
        sessions.extend(claude_sessions);
    }

    // Scan OpenCode sessions
    if let Ok(opencode_sessions) = scan_opencode_sessions_internal(&env) {
        sessions.extend(opencode_sessions);
    }

    // Sort by updated_at descending (most recent first)
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(sessions)
}

/// Get default source paths for display in settings
pub fn get_default_paths() -> Vec<(String, String)> {
    let env = RuntimeEnvironment::detect();
    let mut paths = Vec::new();

    if let Some(claude) = env.get_claude_base() {
        paths.push(("Claude".to_string(), claude.to_string_lossy().to_string()));
    }
    if let Some(opencode) = env.get_opencode_base() {
        paths.push(("OpenCode".to_string(), opencode.to_string_lossy().to_string()));
    }

    paths
}

use crate::SourcePath;

pub fn scan_with_custom_paths(paths: &[SourcePath]) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    for source in paths {
        if !source.enabled {
            continue;
        }

        let base_path = PathBuf::from(&source.path);
        if !base_path.exists() {
            continue;
        }

        if source.source_type == "Claude" {
            if let Ok(claude_sessions) = scan_claude_custom(&base_path, &source.name) {
                sessions.extend(claude_sessions);
            }
        } else if source.source_type == "OpenCode" {
            if let Ok(opencode_sessions) = scan_opencode_custom(&base_path) {
                sessions.extend(opencode_sessions);
            }
        }
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

fn scan_claude_custom(base_path: &PathBuf, project_name: &str) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    if !base_path.exists() {
        return Ok(sessions);
    }

    let projects = fs::read_dir(base_path).map_err(|e| e.to_string())?;

    for project in projects.flatten() {
        let project_path = project.path();
        if !project_path.is_dir() {
            continue;
        }

        let proj_name = if project_name.is_empty() {
            project_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            project_name.to_string()
        };

        let sessions_dir = project_path.join("sessions");
        if !sessions_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&sessions_dir) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                    let session = Session::new(
                        file_path.to_string_lossy().to_string(),
                        SessionSource::Claude,
                        project_path.clone(),
                        proj_name.clone(),
                    );
                    sessions.push(session);
                }
            }
        }
    }

    Ok(sessions)
}

fn scan_opencode_custom(base_path: &PathBuf) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    if !base_path.exists() {
        return Ok(sessions);
    }

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().map(|e| e == "json").unwrap_or(false) {
                let session = Session::new(
                    file_path.to_string_lossy().to_string(),
                    SessionSource::OpenCode,
                    base_path.clone(),
                    "opencode".to_string(),
                );
                sessions.push(session);
            }
        }
    }

    Ok(sessions)
}

use crate::SourcePath;

pub fn scan_with_custom_paths(paths: &[SourcePath]) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    for source in paths {
        if !source.enabled {
            continue;
        }

        let base_path = PathBuf::from(&source.path);
        if !base_path.exists() {
            continue;
        }

        if source.source_type == "Claude" {
            if let Ok(claude_sessions) = scan_claude_dir(&base_path, &source.name) {
                sessions.extend(claude_sessions);
            }
        } else if source.source_type == "OpenCode" {
            if let Ok(opencode_sessions) = scan_opencode_dir(&base_path) {
                sessions.extend(opencode_sessions);
            }
        }
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

fn scan_claude_dir(base_path: &PathBuf, project_name: &str) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    if !base_path.exists() {
        return Ok(sessions);
    }

    let projects = fs::read_dir(base_path).map_err(|e| e.to_string())?;

    for project in projects.flatten() {
        let project_path = project.path();
        if !project_path.is_dir() {
            continue;
        }

        let proj_name = if project_name.is_empty() {
            project_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            project_name.to_string()
        };

        let sessions_dir = project_path.join("sessions");
        if !sessions_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&sessions_dir) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                    let session = Session::new(
                        file_path.to_string_lossy().to_string(),
                        SessionSource::Claude,
                        project_path.clone(),
                        proj_name.clone(),
                    );
                    sessions.push(session);
                }
            }
        }
    }

    Ok(sessions)
}

fn scan_opencode_dir(base_path: &PathBuf) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();

    if !base_path.exists() {
        return Ok(sessions);
    }

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().map(|e| e == "json").unwrap_or(false) {
                let session = Session::new(
                    file_path.to_string_lossy().to_string(),
                    SessionSource::OpenCode,
                    base_path.clone(),
                    "opencode".to_string(),
                );
                sessions.push(session);
            }
        }
    }

    Ok(sessions)
}

/// Scans Claude session directories using the provided environment
fn scan_claude_sessions_internal(env: &RuntimeEnvironment) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let claude_base = env.get_claude_base().ok_or("Cannot determine Claude base path")?;

    if !claude_base.exists() {
        return Ok(sessions);
    }

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

        if let Ok(entries) = fs::read_dir(&sessions_dir) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map(|e| e == "jsonl").unwrap_or(false) {
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

/// Scans OpenCode session directories using the provided environment
fn scan_opencode_sessions_internal(env: &RuntimeEnvironment) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let opencode_base = env.get_opencode_base().ok_or("Cannot determine OpenCode base path")?;

    if !opencode_base.exists() {
        return Ok(sessions);
    }

    if let Ok(entries) = fs::read_dir(&opencode_base) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().map(|e| e == "json").unwrap_or(false) {
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

/// Scans Claude session directories (legacy API)
pub fn scan_claude_sessions() -> Result<Vec<Session>, String> {
    let env = RuntimeEnvironment::detect();
    scan_claude_sessions_internal(&env)
}

/// Scans OpenCode session directories (legacy API)
pub fn scan_opencode_sessions() -> Result<Vec<Session>, String> {
    let env = RuntimeEnvironment::detect();
    scan_opencode_sessions_internal(&env)
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
