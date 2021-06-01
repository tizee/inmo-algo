use crate::fetcher::Problem;
pub fn get_problem_links(slug_title: &str) -> (String, String) {
    (
        format!("https://leetcode.com/problems/{}/", slug_title),
        format!(
            "https://leetcode.com/problems/{}/disucss/?currentPage=1&orderBy=most_votes",
            slug_title
        ),
    )
}

pub fn get_template(problem: &Problem, template_file: &String) -> String {
    let (problem_link, discussion_link) = get_problem_links(problem.title.clone().as_str());
    let problem_boilerplate = &template_file
        .replace("TITLE", &problem.title)
        .replace("CONTENT", &problem.content)
        .replace("DISCUSSION_LINK", &discussion_link)
        .replace("PROBLEM_LINK", &problem_link)
        .replace("ID", &problem.question_id.to_owned().to_string())
        .replace(
            "DEFAULT_CODE",
            &problem
                .code_definition
                .iter()
                .find(|&code| {
                    return &code.value == "rust";
                })
                .unwrap_or_else(|| panic!("Failed to find default code for Rust"))
                .default_code,
        );
    return problem_boilerplate.to_owned();
}
