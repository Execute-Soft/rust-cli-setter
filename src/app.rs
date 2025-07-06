use clap::Parser;
use colored::*;
use log::info;

use crate::cli::{Cli, Commands};
use crate::commands::{hello, version};
use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Set log level based on verbosity
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    match cli.command {
        Some(Commands::Hello { name }) => {
            hello::run(name).await?;
        }
        Some(Commands::Version) => {
            version::run().await?;
        }
        None => {
            // Default behavior when no subcommand is provided
            println!("{}", "Welcome to CLI Starter!".green().bold());
            println!("Use --help to see available commands.");
        }
    }

    info!("CLI application completed successfully.");
    Ok(())
}
