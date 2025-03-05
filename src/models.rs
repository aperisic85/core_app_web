use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub peer_addr: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
