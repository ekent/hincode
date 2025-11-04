//! CLI 参数定义

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hincode")]
#[command(about = "AI-powered code review tool", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Base branch to compare against
    #[arg(short, long, default_value = "main")]
    pub base: String,

    /// Output format (terminal, markdown, json)
    #[arg(short, long, default_value = "terminal")]
    pub output: String,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
