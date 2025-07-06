use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cli-starter",
    about = "A CLI application starter template",
    version,
    long_about = "A comprehensive CLI application starter template with subcommands, error handling, and logging."
)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// The command to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Say hello to someone
    Hello {
        /// The name to greet
        #[arg(default_value = "World")]
        name: String,
    },
    /// Show version information
    Version,
}
