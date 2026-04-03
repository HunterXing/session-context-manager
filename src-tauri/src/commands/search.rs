use crate::models::Session;
use chrono::{DateTime, Utc};

/// Search sessions by case-insensitive substring match on message content.
/// Returns references to sessions that contain the query in any message.
pub fn search_sessions<'a>(sessions: &'a [Session], query: &str) -> Vec<&'a Session> {
    if query.is_empty() {
        return sessions.iter().collect();
    }

    let query_lower = query.to_lowercase();
    sessions
        .iter()
        .filter(|session| {
            session
                .messages
                .iter()
                .any(|msg| msg.content.to_lowercase().contains(&query_lower))
        })
        .collect()
}

/// Filter sessions by date range (inclusive on both bounds).
/// If start is None, no lower bound is applied.
/// If end is None, no upper bound is applied.
pub fn filter_by_date_range<'a>(
    sessions: &'a [Session],
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> Vec<&'a Session> {
    sessions
        .iter()
        .filter(|session| {
            let after_start = start.map_or(true, |s| session.started_at >= s);
            let before_end = end.map_or(true, |e| session.started_at <= e);
            after_start && before_end
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Message, SessionSource};
    use std::path::PathBuf;

    fn create_test_session(
        id: &str,
        started_at: DateTime<Utc>,
        messages_content: Vec<&str>,
    ) -> Session {
        let mut session = Session::new(
            id.to_string(),
            SessionSource::Claude,
            PathBuf::from("/test/path"),
            "test_project".to_string(),
        );
        session.started_at = started_at;
        session.messages = messages_content
            .into_iter()
            .map(|content| Message {
                role: "user".to_string(),
                content: content.to_string(),
                timestamp: started_at,
            })
            .collect();
        session
    }

    #[test]
    fn test_empty_query_returns_all_sessions() {
        let session1 = create_test_session("1", Utc::now(), vec!["hello"]);
        let session2 = create_test_session("2", Utc::now(), vec!["world"]);
        let sessions = vec![session1, session2];

        let result = search_sessions(&sessions, "");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_search_case_insensitive() {
        let session1 = create_test_session("1", Utc::now(), vec!["Hello World"]);
        let session2 = create_test_session("2", Utc::now(), vec!["Goodbye"]);
        let sessions = vec![session1, session2];

        let result = search_sessions(&sessions, "hello");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "1");

        let result_upper = search_sessions(&sessions, "HELLO");
        assert_eq!(result_upper.len(), 1);
        assert_eq!(result_upper[0].id, "1");
    }

    #[test]
    fn test_search_matches_message_content() {
        let session1 =
            create_test_session("1", Utc::now(), vec!["first message", "second message"]);
        let session2 = create_test_session("2", Utc::now(), vec!["different content"]);
        let sessions = vec![session1, session2];

        let result = search_sessions(&sessions, "second");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "1");
    }

    #[test]
    fn test_filter_by_date_range_inclusive() {
        let base_time = Utc::now();
        let session1 = create_test_session("1", base_time, vec!["old"]);
        let session2 = create_test_session("2", base_time, vec!["middle"]);
        let session3 = create_test_session("3", base_time, vec!["new"]);
        let sessions = vec![session1, session2, session3];

        let result = filter_by_date_range(&sessions, Some(base_time), Some(base_time));
        assert_eq!(result.len(), 3);

        let result = filter_by_date_range(&sessions, Some(base_time), None);
        assert_eq!(result.len(), 3);

        let result = filter_by_date_range(&sessions, None, Some(base_time));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_combined_search_and_filter() {
        let base_time = Utc::now();
        let session1 = create_test_session("1", base_time, vec!["apple pie"]);
        let session2 = create_test_session("2", base_time, vec!["banana bread"]);
        let session3 = create_test_session("3", base_time, vec!["apple crisp"]);
        let sessions: Vec<Session> = vec![session1, session2, session3];

        let date_filtered = filter_by_date_range(&sessions, Some(base_time), Some(base_time));
        assert_eq!(date_filtered.len(), 3);

        let searched: Vec<&Session> = date_filtered
            .into_iter()
            .filter(|s| s.messages.iter().any(|m| m.content.contains("apple")))
            .collect();
        assert_eq!(searched.len(), 2);
        assert_eq!(searched[0].id, "1");
        assert_eq!(searched[1].id, "3");
    }
}
