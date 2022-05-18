mod fetcher;
mod problem;
mod query;
mod storage;
mod table;
mod template;

use anyhow::anyhow;
use anyhow::{Context, Result};
use rand::Rng;
use regex::Regex;

use std::fmt::Display;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread;
use std::time::SystemTime;
use std::{fs::File, io};

use std::env;
use std::process::{Command, Stdio};

use fetcher::LCFetcher;
use lazy_static::lazy_static;
use problem::{
    LCEdge, LCProblem, LCQuestionDetail, LCQuestionTopicTag, LCSimilarQuestion, LCTopicTag, Problem,
};
use template::TemplateBuilder;

use crate::common::Lang;
use crate::common::TreeView;

pub struct ProblemEntry {
    pub id: u32,
    pub title: String,
    pub level: String,
    pub langs: Vec<Lang>,
    pub topics: Vec<String>,
}

// display for tree view
impl Display for ProblemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let langs_node = self
            .langs
            .iter()
            .map(|lang| TreeView::new(lang.to_string(), None))
            .collect::<Vec<TreeView>>();
        let root = TreeView::new(
            format!("{:04}\t{}\t{}", self.id, self.level, self.title),
            Some(langs_node),
        );
        f.write_fmt(format_args!("{}", root.draw_default(2)))
    }
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

    #[inline]
    fn cache_problem_list(&self) -> PathBuf {
        self.cache_dir.join("leetcode-problems")
    }

    #[inline]
    fn cache_problem_dir(&self) -> PathBuf {
        self.cache_dir.join("lc-problem")
    }

    #[inline]
    fn cache_problem(&self, problem_id: u32) -> PathBuf {
        self.cache_problem_dir().join(problem_id.to_string())
    }

    #[inline]
    fn cache_tags(&self) -> PathBuf {
        self.cache_dir.join("lc-tags")
    }

    #[inline]
    fn cache_tag_problems(&self, topic: &String) -> PathBuf {
        self.cache_problem_dir().join(topic)
    }

    /// add problem to todo directory
    /// if the problem has been already added, then it's a no-op
    pub async fn add_todo(&self, front_problem_id: u32, lang: &Lang) -> Result<Option<PathBuf>> {
        let detail = self.get_question_detail(front_problem_id).await?;
        if let Some(question_detail) = detail {
            let problem = question_detail.to_problem();
            return Ok(Some(self.add_todo_problem(lang, &problem).unwrap()));
        }
        eprintln!("{} doesn't exist!!", front_problem_id);
        Ok(None)
    }
    /// get list of todos from todos directory
    pub async fn todos(&self) -> Result<Vec<ProblemEntry>> {
        let files = get_problem_files(self.todo_dir())?;
        let mut res = Vec::new();
        // fetch cache
        for file in files.iter() {
            let q = self.get_question_detail(file.id).await?;
            if let Some(detail) = q {
                res.push(ProblemEntry {
                    id: file.id,
                    title: detail.title.unwrap(),
                    level: detail.difficulty.unwrap(),
                    langs: file.langs.to_owned(),
                    topics: detail
                        .topic_tags
                        .unwrap()
                        .iter()
                        .map(|tag| tag.slug.to_owned())
                        .collect(),
                });
            }
        }
        Ok(res)
    }

    pub async fn solutions(&self) -> Result<Vec<ProblemEntry>> {
        let files = get_problem_files(self.solved_dir())?;
        let mut res = Vec::new();
        // fetch cache
        for file in files.iter() {
            let q = self.get_question_detail(file.id).await?;
            if let Some(detail) = q {
                res.push(ProblemEntry {
                    id: file.id,
                    title: detail.title.unwrap(),
                    level: detail.difficulty.unwrap(),
                    langs: file.langs.to_owned(),
                    topics: detail
                        .topic_tags
                        .unwrap()
                        .iter()
                        .map(|tag| tag.slug.to_owned())
                        .collect(),
                });
            }
        }
        Ok(res)
    }

    /// complete todo solution by moving it to solutions directory
    pub fn solve_todo(&self, problem_id: u32, lang: &Lang) -> Result<()> {
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
            println!(
                "move {} to {}",
                problem_file_path.display(),
                solution_file_path.display()
            );

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
        } else {
            eprintln!("{} doesn't exist!", problem_file_path.display());
        }
        Ok(())
    }

    async fn get_question_detail(&self, front_problem_id: u32) -> Result<Option<LCQuestionDetail>> {
        let cache_problem = self.cache_problem(front_problem_id);
        let cache_problem_dir = self.cache_problem_dir();
        if !cache_problem_dir.exists() {
            fs::create_dir_all(cache_problem_dir)?;
        }
        let detail: Option<LCQuestionDetail>;
        if cache_problem.exists() {
            detail = Some(storage::Storage::load_data_from_file(cache_problem)?);
        } else {
            let list = self.get_questions().await?;
            let problem = list
                .iter()
                .find(|&p| !p.paid_only && p.stat.frontend_question_id == front_problem_id);
            if problem.is_some() {
                let problem = problem.unwrap();
                let title_slug = problem.stat.question_title_slug.clone().unwrap();
                let resp = LCFetcher::download_problem(title_slug).await?;
                storage::Storage::persist(cache_problem, &resp)?;
                detail = Some(resp);
            } else {
                detail = None;
            }
        }
        Ok(detail)
    }

    /// get list of all problems
    /// update every week since LeetCode add new problems every week.
    async fn get_questions(&self) -> Result<Vec<LCProblem>> {
        let cache_file = self.cache_problem_list();
        if cache_file.exists() {
            lazy_static! {
                static ref HOUR: u64 = 60 * 60;
                static ref DAY: u64 = 60 * 60 * 24;
                static ref WEEK: u64 = 7 * 60 * 60 * 24;
            };
            if self.is_list_cache_outdated(*WEEK) {
                let problems = LCFetcher::download_problems().await?;
                self.update_list_cache(&problems.stat_status_pairs)?;
                Ok(problems.stat_status_pairs)
            } else {
                storage::Storage::load_data_from_file(cache_file)
            }
        } else {
            let problems = LCFetcher::download_problems().await?;
            self.update_list_cache(&problems.stat_status_pairs)
                .context("failed to update cache")?;
            Ok(problems.stat_status_pairs)
        }
    }

    fn is_list_cache_outdated(&self, limit: u64) -> bool {
        let cache = self.cache_problem_list();
        if cache.exists() {
            // is older than one week?
            let metadata = fs::metadata(cache).unwrap();
            if let Ok(modified_time) = metadata.modified() {
                let elapsed = SystemTime::now()
                    .duration_since(modified_time)
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
    pub fn update_list_cache(&self, list: &Vec<LCProblem>) -> Result<()> {
        let cache_dir = &self.cache_dir;
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        let cache = self.cache_problem_list();
        storage::Storage::persist(cache, list)
    }

    /// clear downloaded cache
    pub fn clear_cache(&self) -> Result<()> {
        let cache = self.cache_problem_list();
        if cache.exists() {
            fs::remove_file(cache).with_context(|| "failed to clear cache for Leetcode")?;
        }
        Ok(())
    }

    fn add_todo_problem(&self, lang: &Lang, problem: &Problem) -> Result<PathBuf> {
        let todo_dir = self.todo_dir();
        if !todo_dir.exists() {
            fs::create_dir_all(&todo_dir)?;
        }
        let file_name = padding_id(problem.question_id);
        let file_path = self
            .todo_dir()
            .join(format!("{}.{}", file_name, lang.to_extension()));
        let solved_file_path =
            self.solved_dir()
                .join(format!("{}.{}", file_name, lang.to_extension()));
        if solved_file_path.exists() {
            return Err(anyhow!(format!("{} is solved", problem.title)));
        }
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
        } else {
            println!("{} already added!!!", file_path.display());
        }
        Ok(file_path)
    }

    pub async fn filter_problems(&self, query: &SearchCondition) -> Result<Vec<LCQuestionDetail>> {
        if let Some(ref topics) = query.topics {
            let mut problems: Vec<LCQuestionDetail> = Vec::new();
            // get problems for each topics
            for topic in topics.iter() {
                if let Ok(p_list) = self.get_problems_of_tag(topic).await {
                    problems = p_list
                        .questions
                        .into_iter()
                        .chain(problems.into_iter())
                        .collect();
                }
            }
            if let Some(ref level) = query.level {
                Ok(problems
                    .into_iter()
                    .filter(|p| {
                        p.difficulty
                            .as_ref()
                            .unwrap()
                            .to_string()
                            .eq_ignore_ascii_case(level)
                    })
                    .collect())
            } else {
                Ok(problems)
            }
        } else {
            // problems list
            let problems = self.get_questions().await?;
            if let Some(ref level) = query.level {
                Ok(problems
                    .into_iter()
                    .filter_map(|p| {
                        if p.difficulty.to_string().eq_ignore_ascii_case(level) {
                            return Some(p.to_detail());
                        }
                        None
                    })
                    .collect())
            } else {
                Ok(problems.into_iter().map(|p| p.to_detail()).collect())
            }
        }
    }

    /// pick one question based on conditions
    /// for the sake of simplicity, please use web UI to query with compliciated conditions
    pub async fn pick_one(&self, query: SearchCondition) -> Result<Option<PathBuf>> {
        let matches = self.filter_problems(&query).await?;
        if !matches.is_empty() {
            let random_index: usize = rand::thread_rng().gen_range(0..=matches.len());
            let question = &matches[random_index];
            let question_detail = self
                .get_question_detail(
                    question
                        .question_frontend_id
                        .as_ref()
                        .unwrap()
                        .parse()
                        .unwrap(),
                )
                .await?;
            if let Some(detail) = question_detail {
                println!(
                    "pick {} {}",
                    detail.question_frontend_id.as_ref().unwrap(),
                    detail.title_slug.as_ref().unwrap()
                );
                let problem = detail.to_problem();
                return Ok(Some(self.add_todo_problem(&query.lang, &problem).unwrap()));
            }
        } else {
            eprintln!("There is no matched problem!");
        }
        Ok(None)
    }

    /// get tags of a question
    pub async fn get_question_tags(&self, question_id: u32) -> Result<Option<Vec<String>>> {
        let question = self.get_question_detail(question_id).await?;
        if let Some(detail) = question {
            return Ok(Some(
                detail
                    .topic_tags
                    .unwrap()
                    .iter()
                    .map(|t| t.to_string())
                    .collect(),
            ));
        }
        Ok(None)
    }

    pub async fn get_all_tags(&self) -> Result<Vec<LCEdge<LCQuestionTopicTag>>> {
        let cache_file = self.cache_tags();
        if cache_file.exists() {
            lazy_static! {
                static ref HOUR: u64 = 60 * 60;
                static ref DAY: u64 = 60 * 60 * 24;
                static ref WEEK: u64 = 7 * 60 * 60 * 24;
            };
            if self.is_list_cache_outdated(*WEEK) {
                let tags = LCFetcher::download_topic_list().await?;
                storage::Storage::persist(cache_file, &tags)?;
                Ok(tags)
            } else {
                storage::Storage::load_data_from_file(cache_file)
            }
        } else {
            let tags = LCFetcher::download_topic_list().await?;
            storage::Storage::persist(cache_file, &tags)?;
            Ok(tags)
        }
    }

    pub async fn get_problems_of_tag(&self, topic: &String) -> Result<LCTopicTag> {
        let cache_file = self.cache_tag_problems(topic);
        if cache_file.exists() {
            lazy_static! {
                static ref HOUR: u64 = 60 * 60;
                static ref DAY: u64 = 60 * 60 * 24;
                static ref WEEK: u64 = 7 * 60 * 60 * 24;
            };
            if self.is_list_cache_outdated(*WEEK) {
                let problems = LCFetcher::download_tag_questions(topic).await?;
                storage::Storage::persist(cache_file, &problems)?;
                Ok(problems)
            } else {
                storage::Storage::load_data_from_file(cache_file)
            }
        } else {
            let problems = LCFetcher::download_tag_questions(topic).await?;
            storage::Storage::persist(cache_file, &problems)?;
            Ok(problems)
        }
    }

    pub async fn get_similar_questions(
        &self,
        front_problem_id: u32,
    ) -> Result<Option<Vec<LCSimilarQuestion>>> {
        let q = self.get_question_detail(front_problem_id).await?;
        if let Some(detail) = q {
            if let Some(s_str) = detail.similar_questions {
                let q_list = serde_json::from_str::<Vec<LCSimilarQuestion>>(&s_str)?;
                return Ok(Some(q_list));
            }
        }
        Ok(None)
    }
}

pub struct SearchCondition {
    /// langugage is required
    pub lang: Lang,
    pub level: Option<String>,
    pub topics: Option<Vec<String>>,
}

pub struct SearchConditionBuilder {
    pub lang: Lang,
    pub level: Option<String>,
    pub topics: Option<Vec<String>>,
}

impl SearchConditionBuilder {
    pub fn new() -> Self {
        SearchConditionBuilder {
            lang: Lang::Rust,
            level: None,
            topics: None,
        }
    }
    pub fn lang(&mut self, name: Lang) -> &mut Self {
        self.lang = name;
        self
    }

    pub fn level(&mut self, level: String) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn topics(&mut self, topics: Vec<String>) -> &mut Self {
        self.topics = Some(topics);
        self
    }

    pub fn build(self) -> SearchCondition {
        SearchCondition {
            lang: self.lang,
            level: self.level,
            topics: self.topics,
        }
    }
}

#[inline(always)]
fn padding_id(question_id: u32) -> String {
    format!("p{:04}", question_id)
}

struct ProblemFileSet {
    pub id: u32,
    pub langs: Vec<Lang>,
}

use std::collections::HashSet;
#[inline]
fn get_problem_files<P: AsRef<Path>>(p: P) -> Result<Vec<ProblemFileSet>> {
    let path = p.as_ref();
    if path.is_dir() {
        // get list of problem_id
        let paths = fs::read_dir(path).unwrap();
        let mut problem_set = HashSet::new();
        let list: Vec<(u32, Lang)> = paths
            .map(|entry| entry.unwrap().path())
            .filter_map(path_to_id_lang)
            .collect();
        for (id, _) in list.iter() {
            problem_set.insert(*id);
        }
        let mut res = Vec::new();
        for id in problem_set.iter() {
            let langs: Vec<Lang> = list
                .iter()
                .filter_map(|(i, lang)| if i == id { Some(lang.clone()) } else { None })
                .collect();
            res.push(ProblemFileSet { id: *id, langs });
        }
        return Ok(res);
    }
    Err(anyhow!(format!("{} is not a directory", path.display())))
}

#[inline]
fn path_to_id_lang(path: PathBuf) -> Option<(u32, Lang)> {
    let fname = path.file_name().unwrap().to_str().unwrap();
    let pair = fname.split_once('.');
    if let Some(pair) = pair {
        let file_extension = pair.1;
        let id = &pair.0[1..];
        return match id.parse::<u32>() {
            Ok(id) => {
                return Some((id, Lang::from_extension(file_extension)));
            }
            Err(_) => None,
        };
    }
    None
}

#[cfg(test)]
mod test_leetcode {
    use super::path_to_id_lang;
    use super::Lang;
    use std::path::PathBuf;
    #[test]
    fn test_path_to_entry() {
        let p = PathBuf::from("p0001.rs");
        let id_lang = path_to_id_lang(p);
        assert!(id_lang.is_some());
        let id_lang = id_lang.unwrap();
        assert_eq!(id_lang.0, 1);
        assert_eq!(id_lang.1, Lang::Rust);
        let p = PathBuf::from("p0936.py");
        let id_lang = path_to_id_lang(p);
        assert!(id_lang.is_some());
        let id_lang = id_lang.unwrap();
        assert_eq!(id_lang.0, 936);
        assert_eq!(id_lang.1, Lang::Python3);
        let p = PathBuf::from("mod.rs");
        let id_lang = path_to_id_lang(p);
        assert!(id_lang.is_none());
    }
}
