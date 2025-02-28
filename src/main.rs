use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use tokio::task;
use tracing::{info, error};
use std::collections::HashMap;

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

            // Write to file
            if let Err(e) = write_to_file(peer_addr, &headers, &body).await {
                error!("Failed to write to file: {}", e);
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

/// Writes the parsed data to a file
async fn write_to_file(peer_addr: String, headers: &HashMap<String, String>, body: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("connections.log")
        .await?;

    // Format the log entry
    let mut log_entry = format!("\n--- New Request from {} ---\n", peer_addr);
    
    for (key, value) in headers {
        log_entry.push_str(&format!("{}: {}\n", key, value));
    }

    log_entry.push_str(&format!("\nBody:\n{}\n", body));
    log_entry.push_str("\n-------------------------\n");

    // Write to file
    file.write_all(log_entry.as_bytes()).await?;
    
    Ok(())
}
