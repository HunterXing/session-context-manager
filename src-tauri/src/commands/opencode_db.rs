use crate::models::{Message, Session, SessionSource};
use chrono::{TimeZone, Utc};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

static DB_PATH: &str = ".local/share/opencode/opencode.db";

pub struct OpenCodeDB {
    conn: Mutex<Connection>,
}

impl OpenCodeDB {
    pub fn new() -> Result<Self, String> {
        let home = std::env::var("HOME").map_err(|_| "Failed to get HOME")?;
        let db_path = PathBuf::from(&home).join(DB_PATH);
        Self::new_with_path(&db_path)
    }

    pub fn new_with_path(db_path: &PathBuf) -> Result<Self, String> {
        if !db_path.exists() {
            return Err(format!("OpenCode database not found at {:?}", db_path));
        }

        let conn = Connection::open(db_path)
            .map_err(|e| format!("Failed to open OpenCode database: {}", e))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get_all_sessions(&self) -> Result<Vec<Session>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "Failed to lock DB connection")?;

        let mut stmt = conn
            .prepare(
                "SELECT id, title, directory, time_created, time_updated 
             FROM session 
             ORDER BY time_updated DESC",
            )
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let session_iter = stmt
            .query_map([], |row| {
                Ok(SessionRow {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    directory: row.get(2)?,
                    time_created: row.get(3)?,
                    time_updated: row.get(4)?,
                })
            })
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let mut sessions = Vec::new();
        for session_result in session_iter {
            if let Ok(row) = session_result {
                let session = self.row_to_session(&conn, row)?;
                sessions.push(session);
            }
        }

        Ok(sessions)
    }

    fn row_to_session(&self, conn: &Connection, row: SessionRow) -> Result<Session, String> {
        let messages = self.get_messages_for_session(conn, &row.id)?;

        let started_at = messages
            .first()
            .map(|m| m.timestamp)
            .unwrap_or_else(Utc::now);
        let updated_at = messages
            .last()
            .map(|m| m.timestamp)
            .unwrap_or_else(Utc::now);

        let project_name = row.title.clone();
        let project_path = PathBuf::from(&row.directory);

        Ok(Session {
            id: row.id,
            source: SessionSource::OpenCode,
            project_path,
            project_name,
            started_at,
            updated_at,
            messages,
            raw_file_path: PathBuf::new(),
        })
    }

    fn get_messages_for_session(
        &self,
        conn: &Connection,
        session_id: &str,
    ) -> Result<Vec<Message>, String> {
        let mut stmt = conn
            .prepare("SELECT data FROM message WHERE session_id = ? ORDER BY time_created ASC")
            .map_err(|e| format!("Failed to prepare message query: {}", e))?;

        let message_iter = stmt
            .query_map([session_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .map_err(|e| format!("Failed to execute message query: {}", e))?;

        let mut messages = Vec::new();
        for msg_result in message_iter {
            if let Ok(data) = msg_result {
                if let Some(msg) = self.parse_message_data(&data) {
                    messages.push(msg);
                }
            }
        }

        Ok(messages)
    }

    fn parse_message_data(&self, data: &str) -> Option<Message> {
        #[derive(serde::Deserialize)]
        struct OpenCodeMessage {
            role: Option<String>,
            content: Option<serde_json::Value>,
            time: Option<OpenCodeTime>,
        }

        #[derive(serde::Deserialize)]
        struct OpenCodeTime {
            created: Option<i64>,
        }

        let parsed: OpenCodeMessage = serde_json::from_str(data).ok()?;

        let role = parsed.role.unwrap_or_else(|| "unknown".to_string());

        let content = match parsed.content {
            Some(serde_json::Value::String(s)) => s,
            Some(v) => serde_json::to_string(&v).unwrap_or_default(),
            None => String::new(),
        };

        let timestamp = parsed
            .time
            .and_then(|t| t.created)
            .map(|ts| {
                Utc.timestamp_millis_opt(ts)
                    .single()
                    .unwrap_or_else(Utc::now)
            })
            .unwrap_or_else(Utc::now);

        Some(Message {
            role,
            content,
            timestamp,
        })
    }
}

struct SessionRow {
    id: String,
    title: String,
    directory: String,
    time_created: i64,
    time_updated: i64,
}

pub fn get_default_opencode_path() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(DB_PATH))
}
