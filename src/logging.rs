use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use chrono::Local;
use std::collections::HashMap;
use serde_json;
use crate::models::LogEntry;

pub async fn write_json_log(peer_addr: String, headers: HashMap<String, String>, body: String) -> Result<(), std::io::Error> {
    let now = Local::now();
    let timestamp = now.to_rfc3339();
    let log_filename = format!("connections-{}.json", now.format("%Y-%m-%d"));

    let log_entry = LogEntry {
        timestamp,
        peer_addr,
        headers,
        body,
    };

    let json_log = serde_json::to_string(&log_entry)? + "\n";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_filename)
        .await?;

    file.write_all(json_log.as_bytes()).await?;
    Ok(())
}
