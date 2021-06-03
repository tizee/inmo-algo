extern crate tokio;

use fetcher::{fetcher, Problem};
use regex::Regex;
use std::{
    fs,
    io::{stdin, Write},
};
use std::{fs::File, io};
use std::{io::BufRead, path};
mod util;
use util::build_template;
mod fetcher;

// only for fetching problem information
#[tokio::main]
async fn main() {
    println!("Solve or not to solve, that is a question.");
    let mut problem_id = String::new();
    io::stdin()
        .read_line(&mut problem_id)
        .expect("[1;31m\nFailed to read");
    let problem_id: &str = problem_id.trim();
    let problem_id = problem_id
        .parse::<u32>()
        .expect("[1;31m\nFailed to read from command line\n [0m");
    let problem =
        fetcher(problem_id).unwrap_or_else(|| panic!("Failed to get problem #{}", problem_id));
    add_todo_problem(&problem);
}

fn add_todo_problem(problem: &Problem) {
    let file_name = format!("p{:04}_{}", &problem.question_id, &problem.title);
    let file_path = path::Path::new("./src/todo").join(format!("{}.rs", file_name));
    if file_path.exists() {
        println!("Problem {} already exists!", problem.title);
        println!("Has solved it already? [Y/y/N/n]");
        let mut yes_or_not = String::new();
        stdin()
            .read_line(&mut yes_or_not)
            .expect("Failed to read option");
        let yes_or_not = yes_or_not.trim();
        let opt_re = Regex::new(r"(?i)[YN]").unwrap();
        let yes_re = Regex::new(r"(?i)Y").unwrap();
        if opt_re.is_match(yes_or_not) {
            if yes_re.is_match(yes_or_not) {
                move_solve_problem(problem);
            }
        }
        return;
    }
    // template
    let template_file = fs::read_to_string("./template.rs").unwrap_or_else(|err| {
        panic!("Failed to read template.rs {}", err);
    });
    let problem_boilerplate = build_template::get_template(&problem, &template_file);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(problem_boilerplate.as_bytes()).unwrap();
    // release manually
    drop(file);

    let mut todo_mod_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/todo/mod.rs")
        .unwrap();
    writeln!(todo_mod_file, "mod {};", file_name).unwrap_or_else(|err| {
        panic!("Failed to write to todo/mod.rs {}", err);
    });
}

fn move_solve_problem(problem: &Problem) {
    // return if not found
    let problem_file_name = format!("p{:04}_{}", &problem.question_id, &problem.title);
    let problem_file_path = path::Path::new("./src/todo").join(format!("{}.rs", problem_file_name));
    if !problem_file_path.exists() {
        panic!("Problem {} does not exist!", problem_file_name);
    }
    let solution_file_name = format!("s{:04}", &problem.question_id);
    let solution_file_path =
        path::Path::new("./src/solutions").join(format!("{}.rs", solution_file_name));
    if solution_file_path.exists() {
        panic!("Solution {} already exists!", solution_file_name);
    }
    fs::rename(problem_file_path, solution_file_path).unwrap_or_else(|err| panic!("{}", err));
    let p_mod_file = "./src/todo/mod.rs";
    let removed = format!("mod {};", &problem_file_name);
    let lines: Vec<String> = io::BufReader::new(File::open(p_mod_file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| *line != removed)
        .collect();
    fs::write(p_mod_file, lines.join("\n")).unwrap_or_else(|err| panic!("{}", err));
    let mut solution_mod_file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open("./src/solutions/mod.rs")
        .unwrap();
    writeln!(solution_mod_file, "mod {};", solution_file_name).unwrap_or_else(|err| {
        panic!("Failed to write to solutions/mod.rs {}", err);
    });
}

#[test]
fn test_commands() {
    let opt_re = Regex::new(r"^(?i)[YN]$").unwrap();
    let yes_re = Regex::new(r"^(?i)Y$").unwrap();
    assert_eq!(opt_re.is_match(" Y "), false);
    assert_eq!(opt_re.is_match(" y "), false);
    assert_eq!(opt_re.is_match(" N "), false);
    assert_eq!(opt_re.is_match(" n "), false);
    assert_eq!(opt_re.is_match("y"), true);
    assert_eq!(opt_re.is_match("Y"), true);
    assert_eq!(opt_re.is_match("N"), true);
    assert_eq!(opt_re.is_match("n"), true);
    assert_eq!(yes_re.is_match("y"), true);
    assert_eq!(yes_re.is_match("Y"), true);
    assert_eq!(yes_re.is_match(" y "), false);
    assert_eq!(yes_re.is_match(" Y "), false);
}
