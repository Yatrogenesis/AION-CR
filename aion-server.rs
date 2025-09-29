use aion_api::ApiServer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let server = ApiServer::new("127.0.0.1".to_string(), 5950);
    println!("[INFO] Starting AION-CR server on 127.0.0.1:5950");
    server.start().await?;

    Ok(())
}