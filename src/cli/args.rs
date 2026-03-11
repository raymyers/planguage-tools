use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "plg",
    version,
    about = "Navigate and analyze Planguage markdown",
    long_about = None,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show build and version information
    Version,
}
