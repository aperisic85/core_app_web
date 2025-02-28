mod server; // Import the server module

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start_server("0.0.0.0:8080").await?;
    Ok(())
}
