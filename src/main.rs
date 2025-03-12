use clap::{Parser, Subcommand};
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
async fn main() -> Result<(), String> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Server) => {
            run_server()
                .await
                .unwrap();
        }
        Some(Commands::Client) => {
            run_client()
                .await
                .unwrap();
        }
        None => return Err("No command provided".to_string()),
    }

    Ok(())
}
