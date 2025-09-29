use aion_cli::cli::AionCli;
use aion_api::ApiServer;
use clap::Parser;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Check if running in server mode
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--server".to_string()) {
        let server = ApiServer::new("127.0.0.1".to_string(), 5950);
        println!("[INFO] Starting AION-CR server on 127.0.0.1:5950");
        server.start().await?;
    } else {
        let cli = AionCli::parse();
        cli.run().await?;
    }

    Ok(())
}