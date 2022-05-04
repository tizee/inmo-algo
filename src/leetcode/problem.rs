use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct LCProblems {
    pub user_name: String,
    pub num_solved: u32,
    pub num_total: u32,
    pub ac_easy: u32,
    pub ac_hard: u32,
    pub stat_status_pairs: Vec<LCProblem>,
}

/// problem detail in problem list
#[derive(Debug, Serialize, Deserialize)]
pub struct LCProblem {
    pub stat: LCProblemStat,
    pub difficulty: LCDifficulty,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: u32,
    pub progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCDifficulty {
    pub level: u32,
}

#[derive(Debug)]
pub enum LCProblemLevel {
    Easy,
    Medium,
    Hard,
    Unknown,
}

impl From<LCDifficulty> for LCProblemLevel {
    fn from(level: LCDifficulty) -> Self {
        match level.level {
            1 => LCProblemLevel::Easy,
            2 => LCProblemLevel::Medium,
            3 => LCProblemLevel::Hard,
            _ => LCProblemLevel::Unknown,
        }
    }
}

impl Display for LCProblemLevel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            LCProblemLevel::Easy => f.write_str("Easy"),
            LCProblemLevel::Medium => f.write_str("Medium"),
            LCProblemLevel::Hard => f.write_str("Hard"),
            LCProblemLevel::Unknown => f.write_str("Unkonwn"),
        }
    }
}

impl Display for LCDifficulty {
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
pub struct LCProblemStat {
    pub question_id: u32,
    pub frontend_question_id: u32,
    #[serde(rename = "question__article__slug")]
    pub question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    pub question_hide: bool,
    pub total_acs: u32,
    pub total_submitted: u32,
    pub is_new_question: bool,
}

/// generic response structure
#[derive(Debug, Deserialize)]
pub struct LCResp<T> {
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub data: T,
}

/// Problem detail response
pub type LCProblemResp = LCResp<LCQuestionDetailData>;

/// problem detail
#[derive(Debug, Serialize, Deserialize)]
pub struct LCQuestionDetailData {
    pub question: LCQuestionDetail,
}

/// common LeetCode problem detail
#[derive(Debug, Serialize, Deserialize)]
pub struct LCQuestionDetail {
    #[serde(rename = "questionId")]
    pub question_id: Option<String>,
    #[serde(rename = "questionFrontendId")]
    pub question_frontend_id: String,
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    /// may omit in other structure
    pub content: Option<String>,
    #[serde(rename = "isPaidOnly")]
    pub is_paid_only: bool,
    pub difficulty: Option<String>,
    pub stats: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Option<Vec<LCCodeSnippet>>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: Option<String>,
    #[serde(rename = "metaData")]
    pub meta_data: Option<String>,
    #[serde(rename = "topicTags")]
    pub topic_tags: Vec<LCQuestionTopicTag>,
    /// string of json list, need deserialize one more time
    #[serde(rename = "similarQuestions")]
    pub similar_questions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCSimilarQuestion {
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub difficulty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCQuestionTopicTag {
    pub name: String,
    pub slug: String,
}

impl Display for LCQuestionTopicTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.slug)
    }
}

/// Problem used in cli
#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub content: String,
    pub difficulty: Option<String>,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<LCCodeSnippet>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub question_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCCodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

pub type LCTopicTagResp = LCResp<LCTopicTagData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LCTopicTagData {
    #[serde(rename = "topicTag")]
    pub topic_tag: LCTopicTag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCTopicTag {
    pub name: String,
    pub slug: String,
    pub questions: Vec<LCQuestionDetail>,
}

impl Display for LCQuestionDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Level: {}\t{}\t{}",
            self.difficulty.as_ref().unwrap(),
            self.question_frontend_id,
            self.title_slug,
        ))
    }
}

impl LCQuestionDetail {
    /// convert to Problem and move ownership
    pub fn to_problem(self) -> Problem {
        Problem {
            title: self.title_slug,
            content: self.content.unwrap(),
            difficulty: self.difficulty,
            question_id: self.question_frontend_id.parse().unwrap(),
            code_snippets: self.code_snippets.unwrap(),
            sample_test_case: self.sample_test_case.unwrap(),
        }
    }
}

pub type LCQuestionTopicTagsResp = LCResp<LCQuestionTopicTagsData>;

#[derive(Debug, Deserialize)]
pub struct LCQuestionTopicTagsData {
    #[serde(rename = "questionTopicTags")]
    pub question_topic_tags: LCEdges<LCQuestionTopicTag>,
}

#[derive(Debug, Deserialize)]
pub struct LCEdges<T> {
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub edges: Vec<LCEdge<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LCEdge<T> {
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub node: T,
}
