use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task;
use tracing::{info, error};
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
        Ok(0) => return, // Connection closed
        Ok(n) => {
            let request = String::from_utf8_lossy(&buf[..n]);

            // Parse headers and body
            let (headers, body) = parse_request(&request);

            //info!("Parsed Headers: {:?}", headers);
            //info!("Parsed Body: {}", body);

            // Write structured JSON logs
            if let Err(e) = write_json_log(peer_addr, headers, body.clone()).await {
                error!("Failed to write JSON log: {}", e);
            }

            let hacker_face  = r#"
     .-""""""-.
    |   0  0  |  
    |    |  
    |   ----  |  
     '-......-'  
    Hands up, hackers!  

"#;
      
            // Send response
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", hacker_face.len(), hacker_face);
            let _ = socket.write_all(response.as_bytes()).await;
        }
        Err(e) => {
            error!("Error reading from socket: {}", e);
        }
    }
}
