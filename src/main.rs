use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use tokio::task;
use tracing::{info, error};
use std::collections::HashMap;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json;

/// Struct for logging requests as JSON
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    timestamp: String,
    peer_addr: String,
    headers: HashMap<String, String>,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server is running on 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from {}", addr);
                task::spawn(handle_connection(socket, addr.to_string()));
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, peer_addr: String) {
    let mut buf = vec![0; 4096]; // Read up to 4KB

    match socket.read(&mut buf).await {
        Ok(0) => return, // Connection closed
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);

            // Parse headers and body
            let (headers, body) = parse_request(&request);

            info!("Parsed Headers: {:?}", headers);
            info!("Parsed Body: {}", body);

            // Write structured JSON logs
            if let Err(e) = write_json_log(peer_addr, headers, body).await {
                error!("Failed to write JSON log: {}", e);
            }

            // Example: Send response
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            let _ = socket.write_all(response.as_bytes()).await;
        }
        Err(e) => {
            error!("Error reading from socket: {}", e);
        }
    }
}

/// Parses a request string into headers and body
fn parse_request(request: &str) -> (HashMap<String, String>, String) {
    let mut headers = HashMap::new();
    let mut body = String::new();
    
    let mut lines = request.split("\r\n");
    
    // Skip the request line (e.g., "GET / HTTP/1.1")
    if let Some(_) = lines.next() {}

    // Parse headers
    for line in &mut lines {
        if line.is_empty() {
            // Empty line means headers are done, body starts next
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // The rest is the body
    body = lines.collect::<Vec<&str>>().join("\r\n");

    (headers, body)
}

/// Writes structured JSON logs to `connections.json`
async fn write_json_log(peer_addr: String, headers: HashMap<String, String>, body: String) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("connections.json")
        .await?;

    // Get the current timestamp
    let timestamp = Utc::now().to_rfc3339();

    // Create a structured log entry
    let log_entry = LogEntry {
        timestamp,
        peer_addr,
        headers,
        body,
    };

    // Convert to JSON
    let json_log = serde_json::to_string(&log_entry)? + "\n"; // Append newline for easy reading

    // Write to file
    file.write_all(json_log.as_bytes()).await?;

    Ok(())
}
