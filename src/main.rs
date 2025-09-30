use aion_cli::cli::AionCli;
use aion_api::ApiServer;
use clap::Parser;
use tracing_subscriber;
use aion_cr::{start_aion_ectus_unified_system, AionCrSystem};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Check command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--unified".to_string()) || args.contains(&"--ectus-integration".to_string()) {
        // Start unified AION-CR ↔ ECTUS-R system with maximum autonomy
        println!("🚀 Starting unified AION-CR ↔ ECTUS-R system with maximum autonomy");
        let system = start_aion_ectus_unified_system().await?;

        // Keep the system running
        println!("🎉 Unified system operational. Press Ctrl+C to stop.");
        tokio::signal::ctrl_c().await?;

        // Graceful shutdown
        println!("🛑 Shutting down unified system...");
        system.stop().await?;
        println!("✅ System stopped gracefully");

    } else if args.contains(&"--server".to_string()) {
        // Legacy server mode
        let server = ApiServer::new("127.0.0.1".to_string(), 5950);
        println!("[INFO] Starting AION-CR server on 127.0.0.1:5950");
        server.start().await?;
    } else {
        // CLI mode with integration option
        let cli = AionCli::parse();

        // Check if integration is requested via CLI
        if args.contains(&"--enable-ectus".to_string()) {
            println!("🔗 Initializing AION-CR with ECTUS-R integration");
            let system = Arc::new(AionCrSystem::new_with_ectus_integration().await?);

            // Run CLI with integration context
            cli.run_with_integration(system).await?;
        } else {
            // Standard CLI mode
            cli.run().await?;
        }
    }

    Ok(())
}