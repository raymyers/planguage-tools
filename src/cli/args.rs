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
    /// Emit a conversion prompt using input text or a file
    Convert(PromptArgs),
    /// List markdown documents in the workspace
    Get(GetArgs),
    /// Emit a quality-analysis prompt using input text or a file
    Qa(PromptArgs),
    /// Search markdown documents in the workspace
    Search(SearchArgs),
    /// Show markdown repository statistics
    Stats(StatsArgs),
    /// Show the markdown document hierarchy in the workspace
    Tree(TreeArgs),
    /// Show build and version information
    Version,
}

#[derive(Debug, clap::Args)]
pub struct PromptArgs {
    /// Read input content from a file relative to the current workspace
    #[arg(long, conflicts_with = "text")]
    pub file: Option<String>,
    /// Use direct text input instead of a file
    #[arg(long, conflicts_with = "file")]
    pub text: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct GetArgs {
    /// Limit output to markdown files under a relative path prefix
    #[arg(long)]
    pub path_prefix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct SearchArgs {
    /// Text to search for in markdown documents
    pub needle: String,
    /// Limit search to markdown files under a relative path prefix
    #[arg(long)]
    pub path_prefix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct StatsArgs {
    /// Limit stats to markdown files under a relative path prefix
    #[arg(long)]
    pub path_prefix: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct TreeArgs {
    /// Limit the tree to markdown files under a relative path prefix
    #[arg(long)]
    pub path_prefix: Option<String>,
}
