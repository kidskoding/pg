use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};
use pg::{client::run_client, server::run_server};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    match args.command {
        Some(Commands::Server) => {
            run_server()
                .await?;
        }
        Some(Commands::Client) => {
            run_client()
                .await?;
        }
        None => return Err(eyre!("No command provided".to_string())),
    }

    Ok(())
}
