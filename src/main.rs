use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use tokio::task;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing
    tracing_subscriber::fmt::init();

    // Bind the server to an address (e.g., 127.0.0.1:8080)
    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    info!("Server is running on 0.0.0.0:8081");

    // Accept incoming connections
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from {}", addr);

                // Handle each connection in a new task
                task::spawn(handle_connection(socket, addr.to_string()));
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, peer_addr: String) {
    info!("Handling connection from {}", peer_addr);

    // Open the log file in append mode
    let file_result = OpenOptions::new()
        .append(true)
        .create(true)
        .open("connections.log")
        .await;

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open log file: {}", e);
            return;
        }
    };

    let mut buf = vec![0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // Connection closed
                break;
            }
            Ok(n) => {
                // Convert received data to string
                if let Ok(data_str) = String::from_utf8(buf[..n].to_vec()) {
                    info!("Received data: {}", data_str);

                    // Write to log file
                    if let Err(e) = file.write_all(format!("{}: {}\n", peer_addr, data_str).as_bytes()).await {
                        error!("Failed to write to log file: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error reading from socket: {}", e);
                break;
            }
        }
    }

    info!("Connection closed from {}", peer_addr);
}
