use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

mod dashboard;

#[derive(Parser)]
#[command(name = "versionwatch")]
#[command(about = "VersionWatch CLI - Monitor software versions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web dashboard
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind to
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { host, port } => {
            let config = versionwatch_config::load(Path::new("config.yaml"))?;

            println!("🚀 Starting VersionWatch dashboard (React) on http://{host}:{port}");
            dashboard::start_server(&host, port, &config).await?;
        }
    }

    Ok(())
}
