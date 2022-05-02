use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Config;

use super::lc::LeetCodeArgs;

#[derive(Debug, Parser)]
#[clap(long_about=None)]
pub struct App {
    #[clap(subcommand)]
    command: CliCommands,
}

#[derive(Debug, Subcommand)]
enum CliCommands {
    #[clap(name = "leetcode")]
    LeetCode(LeetCodeArgs),
    #[clap(name = "codeforces")]
    CodeForces,
}

impl App {
    pub async fn run(&self, config: &Config) -> Result<()> {
        match &self.command {
            CliCommands::LeetCode(args) => args.run(config).await,
            CliCommands::CodeForces => Ok(()),
        }
    }
}
