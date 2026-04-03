use crate::models::{Session, SessionSource};
use crate::SourcePath;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum RuntimeEnvironment {
    Native,
    WindowsWsl2 {
        ubuntu_home: PathBuf,
        username: String,
    },
    WindowsNative,
}

impl RuntimeEnvironment {
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            let wsl_path = PathBuf::from(r"\\wsl$\Ubuntu");
            if wsl_path.exists() {
                if let Ok(entries) = fs::read_dir(&wsl_path) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.is_dir() {
                            let home_dir = path.join("home");
                            if home_dir.exists() {
                                if let Ok(sub_entries) = fs::read_dir(&home_dir) {
                                    for sub in sub_entries.filter_map(|e| e.ok()) {
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
            }
            RuntimeEnvironment::WindowsNative
        }
        #[cfg(not(target_os = "windows"))]
        {
            RuntimeEnvironment::Native
        }
    }

    pub fn get_home(&self) -> Option<PathBuf> {
        match self {
            RuntimeEnvironment::Native => std::env::var("HOME").ok().map(PathBuf::from),
            RuntimeEnvironment::WindowsWsl2 { ubuntu_home, .. } => Some(ubuntu_home.clone()),
            RuntimeEnvironment::WindowsNative => {
                std::env::var("USERPROFILE").ok().map(PathBuf::from)
            }
        }
    }

    pub fn get_claude_base(&self) -> Option<PathBuf> {
        self.get_home().map(|h| h.join(".claude/projects"))
    }

    pub fn get_opencode_base(&self) -> Option<PathBuf> {
        self.get_home().map(|h| h.join(".config/opencode/sessions"))
    }
}

fn expand_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(path.trim_start_matches('~'));
        }
    }
    PathBuf::from(path)
}

fn escape_project_name(project_path: &str) -> String {
    project_path
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "_")
        .replace(' ', "_")
}

pub fn scan_sources() -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let env = RuntimeEnvironment::detect();

    if let Ok(claude_sessions) = scan_claude_env(&env) {
        sessions.extend(claude_sessions);
    }
    if let Ok(opencode_sessions) = scan_opencode_env(&env) {
        sessions.extend(opencode_sessions);
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

pub fn get_default_paths() -> Vec<(String, String)> {
    let env = RuntimeEnvironment::detect();
    let mut paths = Vec::new();
    if let Some(claude) = env.get_claude_base() {
        paths.push(("Claude".to_string(), claude.to_string_lossy().to_string()));
    }
    if let Some(opencode) = env.get_opencode_base() {
        paths.push((
            "OpenCode".to_string(),
            opencode.to_string_lossy().to_string(),
        ));
    }
    paths
}

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
            if let Ok(opencode_sessions) = scan_opencode_dir(&base_path) {
                sessions.extend(opencode_sessions);
            }
        }
    }
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

fn scan_claude_env(env: &RuntimeEnvironment) -> Result<Vec<Session>, String> {
    let Some(claude_base) = env.get_claude_base() else {
        return Ok(Vec::new());
    };
    if !claude_base.exists() {
        return Ok(Vec::new());
    }
    let mut sessions = Vec::new();
    if let Ok(projects) = fs::read_dir(&claude_base) {
        for project in projects.filter_map(|e| e.ok()) {
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
                for entry in entries.filter_map(|e| e.ok()) {
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
    }
    Ok(sessions)
}

fn scan_opencode_env(env: &RuntimeEnvironment) -> Result<Vec<Session>, String> {
    let Some(opencode_base) = env.get_opencode_base() else {
        return Ok(Vec::new());
    };
    if !opencode_base.exists() {
        return Ok(Vec::new());
    }
    let mut sessions = Vec::new();
    if let Ok(entries) = fs::read_dir(&opencode_base) {
        for entry in entries.filter_map(|e| e.ok()) {
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

fn scan_claude_custom(base_path: &PathBuf, project_name: &str) -> Result<Vec<Session>, String> {
    if !base_path.exists() {
        return Ok(Vec::new());
    }
    let mut sessions = Vec::new();
    if let Ok(projects) = fs::read_dir(base_path) {
        for project in projects.filter_map(|e| e.ok()) {
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
                for entry in entries.filter_map(|e| e.ok()) {
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
    }
    Ok(sessions)
}

fn scan_opencode_dir(base_path: &PathBuf) -> Result<Vec<Session>, String> {
    if !base_path.exists() {
        return Ok(Vec::new());
    }
    let mut sessions = Vec::new();
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.filter_map(|e| e.ok()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_scan_sources_handles_missing_directories() {
        let result = scan_sources();
        assert!(result.is_ok());
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
}
