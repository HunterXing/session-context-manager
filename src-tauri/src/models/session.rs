use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Source of the session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionSource {
    Claude,
    OpenCode,
}

/// A single message in a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

/// A complete session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub source: SessionSource,
    pub project_path: PathBuf,
    pub project_name: String,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub raw_file_path: PathBuf,
}

impl Session {
    /// Create a new empty session
    pub fn new(
        id: String,
        source: SessionSource,
        project_path: PathBuf,
        project_name: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            source,
            project_path,
            project_name,
            started_at: now,
            updated_at: now,
            messages: Vec::new(),
            raw_file_path: PathBuf::new(),
        }
    }
}
