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
    /// List markdown documents in the workspace
    Get(GetArgs),
    /// Show build and version information
    Version,
}

#[derive(Debug, clap::Args)]
pub struct GetArgs {
    /// Limit output to markdown files under a relative path prefix
    #[arg(long)]
    pub path_prefix: Option<String>,
}
