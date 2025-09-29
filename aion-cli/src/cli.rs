use clap::{Parser, Subcommand};
use crate::commands::*;
use aion_core::AionResult;

#[derive(Parser)]
#[command(name = "aion-cr")]
#[command(about = "AION-CR: Artificial Intelligence Operations Network - Compliance & Regulations")]
#[command(version = "1.0.0")]
#[command(author = "AION Development Team")]
pub struct AionCli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[arg(long, global = true)]
    pub config: Option<String>,

    #[arg(long, global = true)]
    pub database_url: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Framework(FrameworkCommand),
    Compliance(ComplianceCommand),
    Conflict(ConflictCommand),
    Audit(AuditCommand),
    Database(DatabaseCommand),
    Server(ServerCommand),
    Init(InitCommand),
}

impl AionCli {
    pub async fn run(self) -> AionResult<()> {
        if self.verbose {
            tracing::info!("Running AION-CR in verbose mode");
        }

        match self.command {
            Commands::Framework(cmd) => cmd.execute().await,
            Commands::Compliance(cmd) => cmd.execute().await,
            Commands::Conflict(cmd) => cmd.execute().await,
            Commands::Audit(cmd) => cmd.execute().await,
            Commands::Database(cmd) => cmd.execute().await,
            Commands::Server(cmd) => cmd.execute().await,
            Commands::Init(cmd) => cmd.execute().await,
        }
    }
}