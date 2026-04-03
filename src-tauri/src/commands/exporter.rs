use std::fs;
use std::path::{Path, PathBuf};

use crate::models::session::{Message, Session, SessionSource};

pub fn export_to_markdown(session: &Session, output_dir: &Path) -> Result<PathBuf, String> {
    let project_dir = output_dir.join(&session.project_name);
    fs::create_dir_all(&project_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    let source_str = match session.source {
        SessionSource::Claude => "claude",
        SessionSource::OpenCode => "opencode",
    };
    let date_str = session.started_at.format("%Y-%m-%d").to_string();
    let filename = format!("{}-{}.md", date_str, source_str);
    let file_path = project_dir.join(&filename);

    let frontmatter = format!(
        "---\n\
         title: \"{} - {}\"\n\
         date: {}\n\
         source: {}\n\
         messages: {}\n\
         ---\n\
         \n\
         # {}\n\
         \n\
         ## Session {}\n\
         \n\
         {}",
        session.project_name,
        date_str,
        session.started_at.to_rfc3339(),
        source_str,
        session.messages.len(),
        session.project_name,
        date_str,
        format_messages(&session.messages)
    );

    // 4. Write the file
    fs::write(&file_path, frontmatter).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(file_path)
}

pub fn export_project(
    project_name: &str,
    sessions: &[Session],
    output_dir: &Path,
) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();

    for session in sessions {
        if session.project_name == project_name {
            let path = export_to_markdown(session, output_dir)?;
            paths.push(path);
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;

    fn create_test_session() -> Session {
        Session {
            id: "test-id".into(),
            source: SessionSource::Claude,
            project_path: PathBuf::from("/test/project"),
            project_name: "test-project".into(),
            started_at: Utc::now(),
            updated_at: Utc::now(),
            messages: vec![
                Message {
                    role: "user".into(),
                    content: "Hello, how are you?".into(),
                    timestamp: Utc::now(),
                },
                Message {
                    role: "assistant".into(),
                    content: "I'm doing great, thank you!".into(),
                    timestamp: Utc::now(),
                },
            ],
            raw_file_path: PathBuf::new(),
        }
    }

    #[test]
    fn test_single_session_export_creates_file() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let session = create_test_session();

        let result = export_to_markdown(&session, temp_dir.path());
        assert!(result.is_ok(), "Export should succeed: {:?}", result);

        let path = result.unwrap();
        assert!(path.exists(), "File should exist at {:?}", path);
    }

    #[test]
    fn test_single_session_export_content_has_frontmatter() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let session = create_test_session();

        let path = export_to_markdown(&session, temp_dir.path()).unwrap();
        let content = fs::read_to_string(&path).expect("Failed to read file");

        assert!(content.starts_with("---"), "Should start with frontmatter");
        assert!(
            content.contains("title:"),
            "Should have title in frontmatter"
        );
        assert!(content.contains("date:"), "Should have date in frontmatter");
        assert!(
            content.contains("source:"),
            "Should have source in frontmatter"
        );
        assert!(
            content.contains("messages:"),
            "Should have messages count in frontmatter"
        );
    }

    #[test]
    fn test_single_session_export_content_has_messages() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let session = create_test_session();

        let path = export_to_markdown(&session, temp_dir.path()).unwrap();
        let content = fs::read_to_string(&path).expect("Failed to read file");

        assert!(
            content.contains("# test-project"),
            "Should have project heading"
        );
        assert!(content.contains("### user"), "Should have user role");
        assert!(
            content.contains("Hello, how are you?"),
            "Should have user message"
        );
        assert!(
            content.contains("### assistant"),
            "Should have assistant role"
        );
        assert!(
            content.contains("I'm doing great"),
            "Should have assistant message"
        );
    }

    #[test]
    fn test_multiple_sessions_export_creates_multiple_files() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let sessions = vec![create_test_session(), {
            let mut s = create_test_session();
            s.id = "test-id-2".into();
            s.source = SessionSource::OpenCode;
            s
        }];

        let result = export_project("test-project", &sessions, temp_dir.path());
        assert!(result.is_ok(), "Export should succeed: {:?}", result);

        let paths = result.unwrap();
        assert_eq!(paths.len(), 2, "Should create 2 files");
    }

    #[test]
    fn test_filename_format() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let session = create_test_session();

        let path = export_to_markdown(&session, temp_dir.path()).unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();

        assert!(
            filename.ends_with("-claude.md"),
            "Filename should end with '-claude.md', got: {}",
            filename
        );
    }

    #[test]
    fn test_export_creates_project_directory() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let session = create_test_session();

        export_to_markdown(&session, temp_dir.path()).unwrap();

        let project_dir = temp_dir.path().join("test-project");
        assert!(
            project_dir.is_dir(),
            "Project directory should exist at {:?}",
            project_dir
        );
    }
}
