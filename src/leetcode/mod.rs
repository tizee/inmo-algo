mod fetcher;
mod problem;
mod query;
mod storage;
mod template;

use anyhow::anyhow;
use anyhow::{Context, Result};
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::SystemTime;
use std::{fs::File, io};

use std::env;
use std::process::{Command, Stdio};

use crate::common::Lang;
use fetcher::LCFetcher;
use lazy_static::lazy_static;
use problem::{LCProblem, Problem};
use template::TemplateBuilder;

pub struct ProblemEntry {
    pub id: u32,
    pub lang: Lang,
}

pub struct LeetCode {
    pub workspace: PathBuf,
    pub cache_dir: PathBuf,
}

impl LeetCode {
    pub fn new(workspace: PathBuf, cache_dir: PathBuf) -> Self {
        LeetCode {
            workspace,
            cache_dir,
        }
    }

    #[inline]
    pub fn todo_dir(&self) -> PathBuf {
        self.workspace.join("src").join("todos")
    }

    #[inline]
    pub fn solved_dir(&self) -> PathBuf {
        self.workspace.join("src").join("solutions")
    }

    fn cache_file(&self) -> PathBuf {
        self.cache_dir.join("leetcode-problems")
    }

    /// add problem to todo directory
    /// if the problem has been already added, then open solution template with $EDITOR
    pub async fn add_todo(&self, problem_id: u32, lang: &Lang) -> Result<()> {
        let list = self.get_problems().await?;
        let res = LCFetcher::fetch(problem_id, &list).await?;
        if let Some(problem) = res {
            self.add_todo_problem(lang, &problem)?
        }
        Ok(())
    }

    /// get list of todos from todos directory
    pub fn todos(&self) -> Result<Vec<ProblemEntry>> {
        self.get_problem_entries(self.todo_dir())
    }

    pub fn solutions(&self) -> Result<Vec<ProblemEntry>> {
        self.get_problem_entries(self.solved_dir())
    }

    fn get_problem_entries<P: AsRef<Path>>(&self, p: P) -> Result<Vec<ProblemEntry>> {
        let path = p.as_ref();
        // get all todos
        if path.is_dir() {
            // get list of problem_id
            let list: Vec<ProblemEntry> = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file())
                .map_while(|path: PathBuf| -> Option<ProblemEntry> {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let _pairs = file_name.split_once('.').unwrap();
                    let file_extension = path.file_stem().unwrap().to_str().unwrap();
                    if file_name != file_extension {
                        return Some(ProblemEntry {
                            id: file_name.parse::<u32>().unwrap(),
                            lang: Lang::from_extension(file_extension),
                        });
                    }
                    None
                })
                .collect();
            return Ok(list);
        }
        Err(anyhow!("todos is not a directory"))
    }

    /// complete todo solution by moving it to solutions directory
    pub fn solve_todo(&self, problem_id: u32, lang: Lang) -> Result<()> {
        let todo_dir = self.todo_dir();
        let solutions_dir = self.solved_dir();
        let file_name = padding_id(problem_id);
        let problem_file = format!("{}.{}", file_name, lang.to_extension());
        let problem_file_path = todo_dir.join(&problem_file);
        let solution_file_path = solutions_dir.join(&problem_file);
        if problem_file_path.exists() {
            if solution_file_path.exists() {
                println!(
                    "Overwrite exisiting solution {}",
                    solution_file_path.display()
                );
                print!("(y/n): ");
                let mut buf = String::new();
                io::stdin()
                    .read_line(&mut buf)
                    .context("failed to parse input")?;
                if !buf.trim().eq_ignore_ascii_case("y") {
                    // terminate immediately
                    return Ok(());
                }
            }
            fs::rename(&problem_file_path, &solution_file_path).context(format!(
                "failed to rename {} to {}",
                problem_file_path.display(),
                solution_file_path.display()
            ))?;
            // update mod file for Rust lang
            if let Lang::Rust = lang {
                let p_mod_file = todo_dir.join("mod.rs");
                let removed = format!("mod {};", file_name);
                let lines: Vec<String> = io::BufReader::new(File::open(&p_mod_file).unwrap())
                    .lines()
                    .map(|line| line.unwrap())
                    .filter(|line| *line != removed)
                    .collect();
                fs::write(&p_mod_file, lines.join("\n")).context(format!(
                    "failed to update {} for {}",
                    p_mod_file.display(),
                    file_name
                ))?;
                let mut solution_mod_file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .write(true)
                    .open(solutions_dir.join("mod.rs"))
                    .unwrap();
                writeln!(solution_mod_file, "mod {};", file_name)?;
            }
        }
        Ok(())
    }

    /// update every week since LeetCode add new problems every week.
    async fn get_problems(&self) -> Result<Vec<LCProblem>> {
        let cache_file = self.cache_file();
        if cache_file.exists() {
            lazy_static! {
                static ref HOUR: u64 = 60 * 60;
                static ref DAY: u64 = 60 * 60 * 24;
                static ref WEEK: u64 = 7 * 60 * 60 * 24;
            };
            if self.is_cache_outdated(*WEEK) {
                let problems = LCFetcher::download_problems().await?;
                self.update_cache(&problems.stat_status_pairs)?;
                Ok(problems.stat_status_pairs)
            } else {
                storage::Storage::load_from_file(cache_file)
            }
        } else {
            let problems = LCFetcher::download_problems().await?;
            self.update_cache(&problems.stat_status_pairs)
                .context("failed to update cache")?;
            Ok(problems.stat_status_pairs)
        }
    }

    fn is_cache_outdated(&self, limit: u64) -> bool {
        let cache = self.cache_file();
        if cache.exists() {
            // is older than one week?
            let metadata = fs::metadata(cache).unwrap();
            if let Ok(create_time) = metadata.created() {
                let elapsed = SystemTime::now()
                    .duration_since(create_time)
                    .unwrap()
                    .as_secs();
                return elapsed > limit;
            } else {
                return false;
            }
        }
        false
    }

    /// update downloaded cache
    pub fn update_cache(&self, list: &Vec<LCProblem>) -> Result<()> {
        let cache_dir = &self.cache_dir;
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        let cache = self.cache_file();
        storage::Storage::persist(cache, list)
    }

    /// clear downloaded cache
    pub fn clear_cache(&self) -> Result<()> {
        let cache = self.cache_file();
        if cache.exists() {
            fs::remove_file(cache).with_context(|| "failed to clear cache for Leetcode")?;
        }
        Ok(())
    }

    fn add_todo_problem(&self, lang: &Lang, problem: &Problem) -> Result<()> {
        let todo_dir = self.todo_dir();
        if !todo_dir.exists() {
            fs::create_dir_all(&todo_dir)?;
        }
        let file_name = padding_id(problem.question_id);
        let file_path = self
            .todo_dir()
            .join(format!("{}.{}", file_name, lang.to_extension()));
        if !file_path.exists() {
            // template
            let template_file = TemplateBuilder::get_template_str(lang);
            let solution_template = template::build_template(problem, lang, &template_file);

            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)
                .unwrap();

            file.write_all(solution_template.as_bytes()).unwrap();

            // drop in thread to speed up
            thread::spawn(move || {
                drop(file);
            });

            if let Lang::Rust = lang {
                let mut todo_mod_file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(todo_dir.join("mod.rs"))
                    .unwrap();

                writeln!(todo_mod_file, "mod {};", file_name).context(format!(
                    "failed to update Rust lang mod file for {}",
                    file_path.display()
                ))?;
            }
        }
        // open in $EDITOR
        // let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        // Command::new(editor)
        //     .arg(file_path)
        //     .spawn()
        //     .context("failed to open in $EDITOR")?;
        Ok(())
    }
}

#[inline(always)]
fn padding_id(question_id: u32) -> String {
    format!("p{:04}", question_id)
}
