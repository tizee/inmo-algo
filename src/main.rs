extern crate tokio;

use fetcher::{fetcher, Problem};
use std::fs::File;
use std::io;
use std::path;
use std::{fs, io::Write};
mod util;
use util::build_template;
mod fetcher;

// only for fetching problem information
#[tokio::main]
async fn main() {
    println!("To do or not to do, that is a question?");
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
    println!("problem = {:?}", problem);
    add_todo_problem(&problem);
}

fn add_todo_problem(problem: &Problem) {
    let file_name = format!("p{:04}_{}", &problem.question_id, &problem.title);
    let file_path = path::Path::new("./src/todo").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("Problem {} already exists!", problem.title);
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

// TODO
fn move_solve_problem(fronted_problem_id: u32) {
    // return if not found
}
