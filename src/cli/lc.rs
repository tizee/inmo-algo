use crate::common::Lang;
use anyhow::Result;
use clap::{Args, Subcommand};

use crate::config::Config;
use crate::leetcode::LeetCode;

#[derive(Debug, Args)]
pub struct LeetCodeArgs {
    /// problem id
    #[clap(short, long, requires = "lang")]
    pub id: Option<u32>,
    /// solve problem
    #[clap(long, requires = "id", requires = "lang")]
    pub solve: bool,
    /// overwrite exisiting solution
    #[clap(short, long, requires = "id")]
    pub write: bool,
    /// show tags of one problem, do not generate template
    #[clap(long, conflicts_with = "write", requires = "id")]
    pub tags: bool,
    /// show languages of one problem, do not generate template
    #[clap(long, conflicts_with = "write", requires = "id")]
    pub langs: bool,
    #[clap(subcommand)]
    pub command: Option<LeetCodeCmds>,
    /// generate template of given language
    #[clap(
        long,
        multiple_occurrences(true),
        requires = "id",
        conflicts_with = "tags",
        conflicts_with = "langs",
        arg_enum
    )]
    pub lang: Lang,
}

#[derive(Debug, Subcommand)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum LeetCodeCmds {
    /// list all types of tags
    Tags,
    /// list all problems of a given tag
    #[clap(arg_required_else_help = true)]
    Tag { tag: String },
}

impl LeetCodeArgs {
    pub async fn run(&self, config: &Config) -> Result<()> {
        let lc = LeetCode::new(config.leetcode.clone(), config.cache.clone());
        let args = self;
        if let Some(id) = args.id {
            lc.add_todo(id, &args.lang).await?;
        } else if let Some(ref command) = args.command {
            match command {
                LeetCodeCmds::Tag { tag } => {
                    if !tag.is_empty() {
                        unimplemented!();
                    }
                }
                LeetCodeCmds::Tags => {}
            }
        }
        Ok(())
    }
}
