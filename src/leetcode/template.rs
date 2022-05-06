use super::problem::Problem;
use crate::common::Lang;
use lazy_static::lazy_static;

#[inline]
pub fn build_problem_links(slug_title: &str) -> (String, String) {
    (
        format!(
            "https://leetcode.com/problems/{}/",
            slug_title.replace('_', "-").as_str()
        ),
        format!(
            "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes",
            slug_title.replace('_', "-").as_str()
        ),
    )
}

pub fn build_template(problem: &Problem, lang: &Lang, template_file: &str) -> String {
    let (problem_link, discussion_link) = build_problem_links(problem.title.as_str());
    let problem_boilerplate = &template_file
        .replace("TITLE", &problem.title)
        .replace("CONTENT", &problem.content)
        .replace("DISCUSSION_LINK", &discussion_link)
        .replace("PROBLEM_LINK", &problem_link)
        .replace("LEVEL", problem.difficulty.as_ref().unwrap())
        .replace("ID", &problem.question_id.to_string())
        .replace(
            "DEFAULT_CODE",
            &problem
                .code_snippets
                .iter()
                .find(|&code| code.lang.eq_ignore_ascii_case(&lang.to_string()))
                .unwrap_or_else(|| panic!("Failed to find default code for Rust"))
                .code,
        );
    problem_boilerplate.to_string()
}

pub struct TemplateBuilder;

impl TemplateBuilder {
    pub(crate) fn get_template_comments(lang: &Lang) -> String {
        lazy_static! {
            static ref C_LIKE_FRONT_MATTER: &'static str = r"/*
 * @frontendId ID
 * @title TITLE
 * @level LEVEL
 *
 * CONTENT
 *
 * @link PROBLEM_LINK
 * @discussion DISCUSSION_LINK
 *
 * */
";
            static ref UNKNOWN_FRONT_MATTER: &'static str = r"
@frontendId ID
@title TITLE
@level LEVEL

CONTENT

@link PROBLEM_LINK
@discussion DISCUSSION_LINK
";
            static ref PYTHON_FRONT_MATTER: &'static str = r#""""
@frontendId ID
@title TITLE
@level LEVEL

CONTENT

@link PROBLEM_LINK
@discussion DISCUSSION_LINK
""""#;
        }
        match lang {
            Lang::Rust => C_LIKE_FRONT_MATTER.to_string(),
            Lang::Cpp => C_LIKE_FRONT_MATTER.to_string(),
            Lang::Python3 => PYTHON_FRONT_MATTER.to_string(),
            Lang::Javascript => C_LIKE_FRONT_MATTER.to_string(),
            Lang::Typescript => C_LIKE_FRONT_MATTER.to_string(),
            Lang::Unknown => UNKNOWN_FRONT_MATTER.to_string(),
        }
    }

    #[inline]
    pub fn get_template_str(lang: &Lang) -> String {
        TemplateBuilder::get_template_comments(lang)
            + "\n"
            + &TemplateBuilder::get_snippet_block(lang)
            + "\n"
            + &TemplateBuilder::get_test_block(lang)
    }

    pub(crate) fn get_test_block(lang: &Lang) -> String {
        match lang {
            Lang::Rust => r"
#[cfg(test)]
mod test_pID {
    use super::*;
    #[test]
    fn test_ID() {}
}
"
            .to_string(),
            Lang::Python3 => r"
def test_solve():
    pass

if __name__== '__main__':
    test_solve()
"
            .to_string(),
            _ => "".to_string(),
        }
    }

    #[inline]
    pub(crate) fn get_snippet_block(lang: &Lang) -> String {
        match lang {
            Lang::Rust => r"
struct Solution;

DEFAULT_CODE
"
            .to_string(),

            _ => r"
DEFAULT_CODE
                "
            .to_string(),
        }
    }
}
