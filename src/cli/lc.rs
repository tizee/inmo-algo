use std::fmt::Display;

use anyhow::Result;
use clap::{ArgEnum, Args, Subcommand};

use crate::common;
use crate::config::Config;
use crate::leetcode::{LeetCode, ProblemEntry, SearchConditionBuilder};
use common::Lang;

#[derive(Debug, Args)]
pub struct LeetCodeArgs {
    /// problem id
    #[clap(short, long)]
    pub id: Option<u32>,
    /// open with $EDITOR
    #[clap(long, requires = "id", requires = "lang")]
    pub open: bool,
    /// solve problem
    #[clap(long, requires = "id", requires = "lang", conflicts_with = "open")]
    pub solve: bool,
    /// show tags of one problem, do not generate template
    #[clap(long, requires = "id", conflicts_with = "open")]
    pub tags: bool,
    /// show related problems
    #[clap(long, requires = "id", conflicts_with = "open")]
    pub related: bool,
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
    pub lang: Option<Vec<Lang>>,
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
    #[clap(multiple_occurrences(true), long)]
    topic: Option<Vec<String>>,
    /// difficulty level
    #[clap(long, arg_enum)]
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
    pub todo: bool,
    /// list local solved
    #[clap(long)]
    pub solved: bool,
}

impl LeetCodeArgs {
    pub async fn run(&self, config: &Config) -> Result<()> {
        let lc = LeetCode::new(config.leetcode.clone(), config.cache.clone());
        let args = self;
        if let Some(id) = args.id {
            if args.solve {
                if let Some(ref langs) = args.lang {
                    for lang in langs.iter() {
                        lc.solve_todo(id, lang)?;
                    }
                } else {
                    lc.solve_todo(id, &config.default_lang)?;
                }
            } else if args.tags || args.related {
                if args.tags {
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
                }
                if args.related {
                    let list = lc.get_similar_questions(id).await?;
                    if let Some(q_list) = list {
                        for q in q_list.iter() {
                            println!("Level: {}\t{}", q.difficulty, q.title_slug);
                        }
                    } else {
                        eprintln!("There is no similar questions for {}", id);
                    }
                }
            } else {
                let mut files = vec![];
                if let Some(ref langs) = args.lang {
                    for lang in langs.iter() {
                        let res = lc.add_todo(id, lang).await?;
                        if let Some(p) = res {
                            files.push(p);
                        }
                    }
                } else {
                    let res = lc.add_todo(id, &config.default_lang).await?;
                    if let Some(p) = res {
                        files.push(p);
                    }
                }
                if args.open {
                    common::open_with_editor(&files);
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
                    if let Some(ref topics) = args.topic {
                        query.topics(topics.to_vec());
                    }
                    let query = query.build();
                    lc.pick_one(query).await?;
                }
                LeetCodeCmds::List(args) => {
                    if args.todo {
                        // list todo
                        let todos = lc.todos().await?;
                        print_entries("Todos", todos);
                    }
                    if args.solved {
                        // list solved
                        let solved = lc.solutions().await?;
                        print_entries("Solved", solved);
                    }
                    if !args.todo && !args.solved {
                        // list todos and solutions
                        let todos = lc.todos().await?;
                        print_entries("Todos", todos);
                        let solved = lc.solutions().await?;
                        print_entries("Solved", solved);
                    }
                }
            }
        }
        Ok(())
    }
}

#[inline]
fn print_entries(title: &str, list: Vec<ProblemEntry>) {
    println!();
    println!("==> {}", title);
    println!();
    if !list.is_empty() {
        for p in list.iter() {
            println!("{}", p);
        }
    } else {
        println!("empty");
    }
}
