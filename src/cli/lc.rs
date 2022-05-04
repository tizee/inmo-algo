use std::fmt::Display;

use anyhow::Result;
use clap::{ArgEnum, Args, Subcommand};

use crate::common::Lang;
use crate::config::Config;
use crate::leetcode::{LeetCode, SearchConditionBuilder};

#[derive(Debug, Args)]
pub struct LeetCodeArgs {
    /// problem id
    #[clap(short, long, requires = "lang")]
    pub id: Option<u32>,
    /// solve problem
    #[clap(long, requires = "id", requires = "lang")]
    pub solve: bool,
    /// show tags of one problem, do not generate template
    #[clap(long, requires = "id")]
    pub tags: bool,
    #[clap(subcommand)]
    pub command: Option<LeetCodeCmds>,
    /// generate template of given language
    #[clap(
        long,
        multiple_occurrences(true),
        requires = "id",
        conflicts_with = "tags",
        arg_enum
    )]
    pub lang: Vec<Lang>,
}

#[derive(Debug, Subcommand)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum LeetCodeCmds {
    /// list local todos or solutions
    List(ListArgs),
    /// list all types of tags
    Tags,
    /// list all problems of a given tag
    #[clap(arg_required_else_help = true)]
    Tag { tag: String },
    /// pick one problem
    #[clap(arg_required_else_help = true)]
    Pick(PickOneArgs),
}

#[derive(Debug, Args)]
pub struct PickOneArgs {
    // TODO: topic
    // #[clap(multiple_occurrences(true), long)]
    // tag: Option<String>,
    /// difficulty level
    #[clap(arg_enum)]
    pub level: Option<LevelEnum>,
    /// generate template of given language
    #[clap(long, arg_enum)]
    pub lang: Lang,
}

#[derive(Debug, Clone, ArgEnum)]
pub enum LevelEnum {
    Easy,
    Medium,
    Hard,
}

impl Display for LevelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelEnum::Easy => f.write_str("easy"),
            LevelEnum::Medium => f.write_str("medium"),
            LevelEnum::Hard => f.write_str("hard"),
        }
    }
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// list local todos
    #[clap(long)]
    pub todo: Option<bool>,
    /// list local solved
    #[clap(long)]
    pub solved: Option<bool>,
}

impl LeetCodeArgs {
    pub async fn run(&self, config: &Config) -> Result<()> {
        let lc = LeetCode::new(config.leetcode.clone(), config.cache.clone());
        let args = self;
        if let Some(id) = args.id {
            if args.solve {
                for lang in args.lang.iter() {
                    lc.solve_todo(id, lang)?;
                }
            } else if args.tags {
                let tags = lc.get_question_tags(id).await?;
                if let Some(tags) = tags {
                    if !tags.is_empty() {
                        println!("tags for {}:", id);
                        for tag in tags.iter() {
                            print!("\t{}", tag);
                        }
                        println!();
                    } else {
                        eprintln!("There is no tag for {}", id);
                    }
                } else {
                    eprintln!("There is no tag for {}", id);
                }
            } else {
                for lang in args.lang.iter() {
                    lc.add_todo(id, lang).await?;
                }
            }
        } else if let Some(ref command) = args.command {
            match command {
                LeetCodeCmds::Tag { tag } => {
                    if !tag.is_empty() {
                        let list = lc.get_problems_of_tag(tag).await?;
                        println!("Topic: {}", list.slug);
                        for p in list.questions.iter() {
                            println!("{}", p);
                        }
                    }
                }
                LeetCodeCmds::Tags => {
                    let tags = lc.get_all_tags().await?;
                    for (i, tag) in tags.iter().enumerate() {
                        println!("\t {} {}", i, &tag.node.to_string());
                    }
                }
                LeetCodeCmds::Pick(args) => {
                    let mut query = SearchConditionBuilder::new();
                    query.lang(args.lang.to_string());
                    if let Some(ref level) = args.level {
                        query.level(level.to_string());
                    }
                    let query = query.build();
                    lc.pick_one(query).await?;
                }
                LeetCodeCmds::List(args) => {
                    if args.todo.is_some() {
                        // list todo
                        let todos = lc.todos()?;
                        println!("Todos:");
                        for todo in todos.iter() {
                            println!("\t {}", todo);
                        }
                    }
                    if args.solved.is_some() {
                        // list solved
                        let solved = lc.solutions()?;
                        println!("Solved:");
                        for solve in solved.iter() {
                            println!("\t {}", solve);
                        }
                    }
                    if args.todo.is_none() && args.solved.is_none() {
                        // list todos and solutions
                        let todos = lc.todos()?;
                        let solved = lc.solutions()?;
                        println!("Todos:");
                        for todo in todos.iter() {
                            println!("\t {}", todo);
                        }
                        println!("Solved:");
                        for solve in solved.iter() {
                            println!("\t {}", solve);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
