use crate::models::{Message, Session, SessionSource};
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct RawMessage {
    role: Option<String>,
    content: Option<String>,
    timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct OpenCodeFormat {
    messages: Vec<RawMessage>,
}

fn timestamp_to_datetime(ts: Option<i64>) -> DateTime<Utc> {
    ts.map(|t| Utc.timestamp_opt(t, 0).single().unwrap_or_else(Utc::now))
        .unwrap_or_else(Utc::now)
}

fn parse_raw_message(raw: RawMessage) -> Message {
    Message {
        role: raw.role.unwrap_or_default(),
        content: raw.content.unwrap_or_default(),
        timestamp: timestamp_to_datetime(raw.timestamp),
    }
}

pub fn parse_claude_jsonl(path: &Path) -> Result<Session, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let messages: Vec<Message> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            match serde_json::from_str::<RawMessage>(trimmed) {
                Ok(raw) => Some(parse_raw_message(raw)),
                Err(_) => None,
            }
        })
        .collect();

    let started_at = messages
        .first()
        .map(|m| m.timestamp)
        .unwrap_or_else(Utc::now);
    let updated_at = messages
        .last()
        .map(|m| m.timestamp)
        .unwrap_or_else(Utc::now);

    let raw_file_path = path.to_path_buf();
    let project_path = path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let project_name = project_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let id = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| uuid_v4());

    Ok(Session {
        id,
        source: SessionSource::Claude,
        project_path,
        project_name,
        started_at,
        updated_at,
        messages,
        raw_file_path,
    })
}

pub fn parse_opencode_json(path: &Path) -> Result<Session, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Ok(create_empty_session(path, SessionSource::OpenCode));
    }

    let opencode_format: OpenCodeFormat =
        serde_json::from_str(trimmed).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let messages: Vec<Message> = opencode_format
        .messages
        .into_iter()
        .map(parse_raw_message)
        .collect();

    let started_at = messages
        .first()
        .map(|m| m.timestamp)
        .unwrap_or_else(Utc::now);
    let updated_at = messages
        .last()
        .map(|m| m.timestamp)
        .unwrap_or_else(Utc::now);

    let raw_file_path = path.to_path_buf();
    let project_path = path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let project_name = project_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let id = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| uuid_v4());

    Ok(Session {
        id,
        source: SessionSource::OpenCode,
        project_path,
        project_name,
        started_at,
        updated_at,
        messages,
        raw_file_path,
    })
}

fn create_empty_session(path: &Path, source: SessionSource) -> Session {
    let raw_file_path = path.to_path_buf();
    let project_path = path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let project_name = project_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let id = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| uuid_v4());
    let now = Utc::now();

    Session {
        id,
        source,
        project_path,
        project_name,
        started_at: now,
        updated_at: now,
        messages: Vec::new(),
        raw_file_path,
    }
}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:032x}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path(fixture_name: &str) -> PathBuf {
        PathBuf::from("tests/fixtures").join(fixture_name)
    }

    #[test]
    fn test_parse_valid_claude_jsonl() {
        let path = fixture_path("valid-claude.jsonl");
        let result = parse_claude_jsonl(&path);
        assert!(result.is_ok(), "Should parse valid Claude JSONL");

        let session = result.unwrap();
        assert_eq!(session.source, SessionSource::Claude);
        assert_eq!(session.messages.len(), 4);

        assert_eq!(session.messages[0].role, "user");
        assert_eq!(session.messages[0].content, "How do I parse JSON in Rust?");
    }

    #[test]
    fn test_parse_valid_opencode_json() {
        let path = fixture_path("valid-opencode.json");
        let result = parse_opencode_json(&path);
        assert!(result.is_ok(), "Should parse valid OpenCode JSON");

        let session = result.unwrap();
        assert_eq!(session.source, SessionSource::OpenCode);
        assert_eq!(session.messages.len(), 4);

        assert_eq!(session.messages[1].role, "assistant");
        assert!(session.messages[1].content.contains("cargo new"));
    }

    #[test]
    fn test_parse_corrupt_jsonl() {
        let path = fixture_path("corrupt.jsonl");
        let result = parse_claude_jsonl(&path);
        assert!(result.is_ok(), "Should handle corrupt JSONL gracefully");

        let session = result.unwrap();
        assert_eq!(session.messages.len(), 2);

        assert_eq!(session.messages[0].role, "user");
        assert_eq!(session.messages[0].content, "Valid line");

        assert_eq!(session.messages[1].role, "assistant");
        assert_eq!(session.messages[1].content, "Another valid line");
    }

    #[test]
    fn test_parse_empty_json() {
        let path = fixture_path("empty.json");
        let result = parse_opencode_json(&path);
        assert!(result.is_ok(), "Should handle empty JSON file");

        let session = result.unwrap();
        assert_eq!(session.messages.len(), 0);
        assert_eq!(session.source, SessionSource::OpenCode);
    }

    #[test]
    fn test_parse_missing_fields_jsonl() {
        let path = fixture_path("missing-fields.jsonl");
        let result = parse_claude_jsonl(&path);
        assert!(result.is_ok(), "Should handle missing fields with defaults");

        let session = result.unwrap();
        assert_eq!(session.messages.len(), 3);

        assert_eq!(session.messages[0].role, "user");
        assert_eq!(session.messages[0].content, "Missing timestamp field");

        assert_eq!(session.messages[1].role, "");
        assert_eq!(session.messages[1].content, "Missing role field");

        assert_eq!(session.messages[2].role, "");
        assert_eq!(session.messages[2].content, "");
    }
}
