use super::problem::{
    LCEdge, LCProblemResp, LCProblems, LCQuestionDetail, LCQuestionTopicTag,
    LCQuestionTopicTagsResp, LCTopicTag, LCTopicTagResp,
};
use super::query::LeetCodeQuery;

use indicatif::ProgressBar;

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
    /// fetch leetcode problem with title slug
    pub async fn download_problem(title_slug: String) -> Result<LCQuestionDetail> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(200);
        pb.set_message(format!("Downloading problem {}....", title_slug));
        let client = reqwest::Client::new();
        let resp = client
            .post(GRAPHQL_API)
            .json(&LeetCodeQuery::build_problem_query(title_slug.as_ref()))
            .send()
            .await?
            .json::<LCProblemResp>()
            .await?;
        let mut question_detail = resp.data.question;
        if let Some(content) = question_detail.content {
            let content = remove_http_tags(&content);
            let content = remove_http_entities(&content);
            question_detail.content = Some(content);
        }
        pb.finish_with_message(format!(
            "{} {} downloaded",
            question_detail.question_frontend_id, title_slug
        ));
        Ok(question_detail)
    }

    /// download all problems
    pub async fn download_problems() -> Result<LCProblems> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(200);
        pb.set_message("Downloading problem lists....");
        let resp = reqwest::get(PROBLEMS_API).await?;
        pb.finish_with_message("lists downloaded");
        Ok(resp.json::<LCProblems>().await?)
    }

    /// download topic list
    pub async fn download_topic_list() -> Result<Vec<LCEdge<LCQuestionTopicTag>>> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(200);
        pb.set_message("Downloading topic list....");
        let client = reqwest::Client::new();
        let resp = client
            .post(GRAPHQL_API)
            .json(&LeetCodeQuery::build_tags_query())
            .send()
            .await?
            .json::<LCQuestionTopicTagsResp>()
            .await?;
        let data = resp.data.question_topic_tags.edges;
        pb.finish_with_message(format!("topic list downloaded, {} in total", data.len()));
        Ok(data)
    }

    /// download questions contains topic
    pub async fn download_tag_questions(topic: &String) -> Result<LCTopicTag> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(200);
        pb.set_message(format!("Downloading problems of topic {}....", topic));
        let client = reqwest::Client::new();
        let resp = client
            .post(GRAPHQL_API)
            .json(&LeetCodeQuery::build_tag_questions_query(topic))
            .send()
            .await?
            .json::<LCTopicTagResp>()
            .await?;
        let data = resp.data.topic_tag;
        pb.finish_with_message(format!("topic {} downloaded", topic));
        Ok(data)
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
