use core_app_web::server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server("0.0.0.0:8082").await?;
    Ok(())
}
