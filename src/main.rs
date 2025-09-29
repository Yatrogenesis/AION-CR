use aion_cli::cli::AionCli;
use clap::Parser;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = AionCli::parse();
    cli.run().await?;

    Ok(())
}