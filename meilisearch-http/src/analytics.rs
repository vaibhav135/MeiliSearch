use segment::client::Client;
use segment::http::HttpClient;
use segment::message::{Message, Track, User};
use serde_json::Value;
use std::fmt::Display;
use uuid::Uuid;

const SEGMENT_API_KEY: &str = "o1xi3wWPlKqSGcO5DxG0PeW0kjeRMyrx";

#[derive(Debug, Clone)]
pub struct Analytics {
    user_id: String,
}

impl Analytics {
    pub fn publish(&self, event_name: String, send: Value) {
        let user = User::UserId {
            user_id: self.user_id.clone(),
        };
        tokio::spawn(async move {
            let client = HttpClient::default();
            let _ = client.send(
                SEGMENT_API_KEY.to_string(),
                Message::Track(Track {
                    user,
                    event: event_name,
                    properties: send,
                    ..Default::default()
                }),
            );
        });
    }
}

impl Default for Analytics {
    fn default() -> Analytics {
        let user_id = Uuid::new_v4().to_string();
        Self { user_id }
    }
}

impl Display for Analytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user_id)
    }
}
