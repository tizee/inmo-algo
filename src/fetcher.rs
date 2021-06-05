extern crate reqwest; // higher-level http client
extern crate serde;
extern crate serde_json;
use futures::executor::block_on;
use regex::Regex;
use std::fmt::{Display, Error, Formatter};
// json serilization
use serde::{Deserialize, Serialize};
use serde_json::json;

const GRAPHQL_API: &str = "https://leetcode.com/graphql";
const PROBLEMS_API: &str = "https://leetcode.com/api/problems/algorithms/";
const PROBLEM_QUERY: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}
"#;

#[derive(Debug, Serialize, Deserialize)]
struct LeetCodeQuery {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl LeetCodeQuery {
    fn build_problem_query(title_slug: &str) -> LeetCodeQuery {
        LeetCodeQuery {
            operation_name: "questionData".to_string(),
            variables: json!({ "titleSlug": title_slug }),
            query: PROBLEM_QUERY.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_hard: u32,
    stat_status_pairs: Vec<ProblemStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemStatus {
    stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unkonwn"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemResp {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub content: String,
    pub difficulty: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub question_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

fn get_problems() -> Result<Problems, Error> {
    let resp = block_on(reqwest::get(PROBLEMS_API));
    return Ok(block_on(resp.unwrap().json::<Problems>()).unwrap());
}

/// fetcher for problem information
pub fn fetcher(problem_id: u32) -> Option<Problem> {
    let problems = get_problems().unwrap();
    for problem in problems.stat_status_pairs.iter() {
        if problem.stat.frontend_question_id == problem_id {
            if problem.paid_only {
                return None;
            }
            let client = reqwest::Client::new();
            let resp: ProblemResp = block_on(
                block_on(
                    client
                        .post(GRAPHQL_API)
                        .json(&LeetCodeQuery::build_problem_query(
                            problem.stat.question_title_slug.as_ref().unwrap(),
                        ))
                        .send(),
                )
                .unwrap()
                .json::<ProblemResp>(),
            )
            .unwrap();
            let re_http_tags = Regex::new(r"(?i)</?[a-z]*>").unwrap();
            let content = re_http_tags
                .replace_all(&resp.data.question.content.as_str(), "")
                .to_string();
            // http entities
            let content = &content
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&nbsp;", " ")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .replace("&minus;", "-")
                .replace("\r\n", "\n")
                .replace("\n\n", "\n")
                .replace("\n", "\n * ")
                .replace("\r", "");
            return Some(Problem {
                title: problem
                    .stat
                    .question_title_slug
                    .clone()
                    .unwrap()
                    .replace("-", "_"),
                content: content.clone(),
                difficulty: problem.difficulty.level.to_string(),
                question_id: problem.stat.frontend_question_id,
                code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
                sample_test_case: resp.data.question.sample_test_case,
            });
        }
    }
    return None;
}

#[test]
fn test_re() {
    let re_http_tags = Regex::new(r"(?i)</*[a-z]*>").unwrap();
    let text = "
 * two_sum
 * Given an array of integers <code>nums</code> and an integer <code>target</code>, return <em>indices of the two numbers such that they add up to <code>target</code></em>.</p>

<p>You may assume that each input would have <strong><em>exactly</em> one solution</strong>, and you may not use the <em>same</em> element twice.</p>

<p>You can return the answer in any order.</p>

<p> </p>
<p><strong>Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,7,11,15], target = 9
<strong>Output:</strong> [0,1]
<strong>Output:</strong> Because nums[0] + nums[1] == 9, we return [0, 1].
</pre>

<p><strong>Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,4], target = 6
<strong>Output:</strong> [1,2]
</pre>

<p><strong>Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,3], target = 6
<strong>Output:</strong> [0,1]
</pre>

<p> </p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 <= nums.length <= 10<sup>4</sup></code></li>
	<li><code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code></li>
	<li><code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code></li>
	<li><strong>Only one valid answer exists.</strong></li>
</ul
<p> </p>
<strong>Follow-up: </strong>Can you come up with an algorithm that is less than <code>O(n<sup>2</sup>) </code>time complexity?
 *
";
    let text = re_http_tags.replace_all(text, "").to_string();
    let text = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&minus;", "-")
        .replace("\r\n", "\n")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ");
    assert_eq!(text, "");
}
