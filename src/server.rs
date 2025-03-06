use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;
use tracing::{error, info};
use std::process::Command;

use crate::logging::write_json_log;
use crate::parser::parse_request;

pub async fn start_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind(address).await?;
    info!("Server is running on {}", address);

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
    let mut buf = vec![0; 4096];

    match socket.read(&mut buf).await {
        Ok(0) => {
            info!("Connection closed by peer: {}", peer_addr);
        },
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);

            // Parse headers, body, and query params
            let (headers, body, query_params) = parse_request(&request);

            // Write structured JSON logs
            if let Err(e) = write_json_log(peer_addr.clone(), headers.clone(), body.clone()).await {
                error!("Failed to write JSON log: {}", e);
            }

            // Generate response based on request
            let response = if let Some(ip) = query_params.get("ping") {
                generate_ping_response(ip)
            } else {
                generate_default_response()
            };

            // Send response
            let _ = socket.write_all(response.as_bytes()).await;
        }
        Err(e) => {
            error!("Error reading from socket: {}", e);
        }
    }
}

fn generate_ping_response(ip: &str) -> String {
    let ping_result = ping_ip(ip);
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        ping_result.len(),
        ping_result
    )
}

fn generate_default_response() -> String {
    let smiley = r#"
    .-""""""-.
  .'          '.
 /   O      O   \
:                :
|    \      /    |
:     '.__..'     :
 \     .-""-.    /
  '.          .'
    '-......-'
HELLO, HACKER!
    "#;

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        smiley.len(),
        smiley
    )
}

fn ping_ip(ip: &str) -> String {
    let output = Command::new("ping")
        .arg("-c")
        .arg("2")
        .arg(ip)
        .output();

    match output {
        Ok(result) => String::from_utf8_lossy(&result.stdout).to_string(),
        Err(_) => "Failed to execute ping command".to_string(),
    }
}