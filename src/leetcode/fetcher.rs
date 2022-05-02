use super::problem::{LCProblem, LCProblemResp, LCProblems, Problem};
use super::query::LeetCodeQuery;

use anyhow::{Context, Result};
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RE_HTTP_TAGS: Regex = Regex::new(r"(?i)</?[a-z]*>").unwrap();
}

static GRAPHQL_API: &str = "https://leetcode.com/graphql";
static PROBLEMS_API: &str = "https://leetcode.com/api/problems/algorithms/";

pub struct LCFetcher;

impl LCFetcher {
    /// fetch leetcode problem with frontend problem id
    pub async fn fetch(problem_id: u32, list: &Vec<LCProblem>) -> Result<Option<Problem>> {
        for problem in list.iter() {
            if problem.stat.frontend_question_id == problem_id {
                if problem.paid_only {
                    return Ok(None);
                }
                let client = reqwest::Client::new();
                let resp = client
                    .post(GRAPHQL_API)
                    .json(&LeetCodeQuery::build_problem_query(
                        problem.stat.question_title_slug.as_ref().unwrap(),
                    ))
                    .send()
                    .await?
                    .json::<LCProblemResp>()
                    .await?;
                let content = remove_http_tags(resp.data.question.content.as_str());
                let content = remove_http_entities(content.as_str());
                return Ok(Some(Problem {
                    title: problem
                        .stat
                        .question_title_slug
                        .clone()
                        .unwrap()
                        .replace('-', "_"),
                    content,
                    difficulty: Some(problem.difficulty.to_string()),
                    question_id: problem.stat.frontend_question_id,
                    code_snippets: resp.data.question.code_snippets,
                    sample_test_case: resp.data.question.sample_test_case,
                }));
            }
        }
        Ok(None)
    }
    /// download all problems
    pub async fn download_problems() -> Result<LCProblems> {
        let resp = reqwest::get(PROBLEMS_API).await?;
        Ok(resp.json::<LCProblems>().await?)
    }
}

#[inline]
fn remove_http_tags(content: &str) -> String {
    RE_HTTP_TAGS.replace_all(content, "").to_string()
}

#[inline]
fn remove_http_entities(content: &str) -> String {
    content
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&minus;", "-")
        .replace("\r\n", "\n")
        .replace("\n\n", "\n")
        .replace('\n', "\n * ")
        .replace('\r', "")
}

#[cfg(test)]
mod test_leetcode_fetcher {
    use super::remove_http_entities;
    use super::RE_HTTP_TAGS;
    // you can't use Reulst<T,E> with #[should_panic] annotation
    #[test]
    fn test_re() {
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
        let text = RE_HTTP_TAGS.replace_all(text, "");
        let text = remove_http_entities(&text);
        assert_eq!(text, "");
    }
}
