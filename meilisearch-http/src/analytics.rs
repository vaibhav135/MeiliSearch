use segment::client::Client;
use segment::http::HttpClient;
use segment::message::{Identify, Message, Track, User};
use serde_json::{json, Value};
use std::fmt::Display;
use sysinfo::DiskExt;
use sysinfo::ProcessorExt;
use sysinfo::System;
use sysinfo::SystemExt;
use uuid::Uuid;

use crate::Opt;

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

impl Analytics {
    pub fn new(opt: Opt) -> Self {
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
            // send an identify event
            let _ = client
            .send(
                SEGMENT_API_KEY.to_string(),
                Message::Identify(Identify {
                    user: user.clone(),
                    traits: json!({
                        "System configuration": {
                            "Distribution": sys.name(),
                            "Kernel Version": sys.kernel_version(),
                            "OS Version": sys.os_version(),
                            "Total RAM (in KB)": sys.total_memory(),
                            "Used RAM (in KB)": sys.used_memory(),
                            "Nb CPUs": sys.processors().len(),
                            "Avg CPU frequency": sys.processors().iter().map(|cpu| cpu.frequency()).sum::<u64>() / sys.processors().len() as u64,
                            "Total disk space (in bytes)": sys.disks().iter().map(|disk| disk.total_space()).sum::<u64>(),
                            "Available memory (in bytes)": sys.disks().iter().map(|disk| disk.available_space()).sum::<u64>(),
                        },
                        "Meilisearch configuration": {
                            "Environment": opt.env.clone(),
                            "Max index size": opt.max_index_size.get_bytes(),
                            "Max udb size": opt.max_udb_size.get_bytes(),
                            "HTTP payload size limit": opt.http_payload_size_limit.get_bytes(),
                            "Snapshot enabled": opt.schedule_snapshot,
                        },
                    }),
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
