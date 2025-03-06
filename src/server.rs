use std::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;
use tracing::{error, info};

use crate::error::AppError;
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

async fn handle_connection(mut socket: tokio::net::TcpStream, peer_addr: String) -> Result<(), AppError> {
    let mut buf = vec![0; 4096];

    match socket.read(&mut buf).await {
        Ok(0) => {
            tracing::info!("Connection closed by peer: {}", peer_addr);
            return Ok(()); // Graceful closure
        },
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);

            // Parse the request
            let (headers, body, query_params) = parse_request(&request)?;

            // Log the request using write_json_log
            write_json_log(peer_addr.clone(), headers.clone(), body.clone()).await?;

            // Generate response based on query params
            let response = if query_params.is_empty() {
                generate_default_response()
            } else if let Some(ip) = query_params.get("ping") {
                generate_ping_response(ip)
            } else {
                // Log the bad query parameters
                tracing::info!("Bad query parameters: {:?}", query_params);
                return Err(AppError::InvalidRequest("Invalid query parameters".to_string()));
            };

            socket.write_all(response.as_bytes()).await?;
            Ok(())
        },
        Err(e) => Err(AppError::IoError(e)),
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
    '-......-
    HELLO, HACKER!
    "#;

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        smiley.len(),
        smiley
    )
}

fn generate_error_response(message: &str) -> String {
    format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", 
            message.len(), 
            message)
}


fn ping_ip(ip: &str) -> String {
    let output = Command::new("ping").arg("-c").arg("2").arg(ip).output();

    match output {
        Ok(result) => String::from_utf8_lossy(&result.stdout).to_string(),
        Err(_) => "Failed to execute ping command".to_string(),
    }
}
