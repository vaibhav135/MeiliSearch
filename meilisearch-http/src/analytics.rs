use segment::client::Client;
use segment::http::HttpClient;
use segment::message::{Identify, Message, Track, User};
use serde_json::{json, Value};
use std::fmt::Display;
use sysinfo::System;
use sysinfo::SystemExt;
use uuid::Uuid;

const SEGMENT_API_KEY: &str = "vHi89WrNDckHSQssyUJqLvIyp2QFITSC";

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
            let _ = client
                .send(
                    SEGMENT_API_KEY.to_string(),
                    Message::Track(Track {
                        user,
                        event: event_name.clone(),
                        properties: send,
                        ..Default::default()
                    }),
                )
                .await;
            println!("ANALYTICS: {} was sent", event_name)
        });
    }

    /*
    pub fn tick(&self, data: Data) {
        self.publish("tick", )
    }
    */
}

impl Default for Analytics {
    fn default() -> Analytics {
        let user_id = Uuid::new_v4().to_string();
        let segment = Self { user_id };
        // segment.publish("Launched for the first time", json!({}))
        let client = HttpClient::default();
        let user = User::UserId {
            user_id: segment.user_id.clone(),
        };
        let mut sys = System::new_all();
        sys.refresh_all();

        tokio::spawn(async move {
            let os = [sys.name(), sys.kernel_version(), sys.os_version()]
                .map(|option| option.unwrap_or_default())
                .join(" ");
            // send an identify event
            let _ = client
                .send(
                    SEGMENT_API_KEY.to_string(),
                    Message::Identify(Identify {
                        user: user.clone(),
                        traits: json!({ "os": os, "total memory": sys.total_memory(), "used memory": sys.used_memory(), "nb cpus": sys.processors().len() }),
                        ..Default::default()
                    }),
                )
                .await;
            println!("ANALYTICS: sent the first identify");

            // send the associated track event
            let _ = client
                .send(
                    SEGMENT_API_KEY.to_string(),
                    Message::Track(Track {
                        user,
                        event: "Launched for the first time".to_string(),
                        ..Default::default()
                    }),
                )
                .await;
            println!("ANALYTICS: sent the first track");
        });
        segment
    }
}

impl Display for Analytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user_id)
    }
}
