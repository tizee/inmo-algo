use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

enum LeetCodeQueryType {
    QuestionData,
    QuestionTags,
}

impl Display for LeetCodeQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LeetCodeQueryType::QuestionData => f.write_str("questionData"),
            LeetCodeQueryType::QuestionTags => f.write_str("questionTags"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeetCodeQuery {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl LeetCodeQuery {
    pub fn build_problem_query(title_slug: &str) -> Self {
        lazy_static! {
            static ref PROBLEM_QUERY: &'static str = r"
query questionData($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    questionId
    questionFrontendId
    boundTopicId
    title
    titleSlug
    content
    translatedTitle
    translatedContent
    isPaidOnly
    difficulty
    likes
    dislikes
    isLiked
    similarQuestions
    exampleTestcases
    categoryTitle
    contributors {
      username
      profileUrl
      avatarUrl
    }
    topicTags {
      name
      slug
      translatedName
    }
    companyTagStats
    codeSnippets {
      lang
      langSlug
      code
    }
    stats
    hints
    solution {
      id
      canSeeDetail
      paidOnly
      hasVideoSolution
      paidOnlyVideo
    }
    status
    sampleTestCase
    metaData
    judgerAvailable
    judgeType
    mysqlSchemas
    enableRunCode
    enableTestMode
    enableDebugger
    envInfo
    libraryUrl
    adminUrl
    challengeQuestion {
      id
      date
      incompleteChallengeCount
      streakCount
      type
    }
  }
}
    ";
        }
        LeetCodeQuery {
            operation_name: LeetCodeQueryType::QuestionData.to_string(),
            variables: json!({ "titleSlug": title_slug }),
            query: PROBLEM_QUERY.to_string(),
        }
    }

    pub fn build_tags_query() -> Self {
        lazy_static! {
            static ref TAGS_QUERY: &'static str = r"query questionTags($skipCompanyTags: Boolean = false) {
  questionTopicTags {
    edges {
      node {
        name
        translatedName
        slug
      }
    }
  }
  questionCompanyTags @skip(if: $skipCompanyTags) {
    edges {
      node {
        name
        translatedName
        slug
      }
    }
  }
}";
        }
        LeetCodeQuery {
            operation_name: LeetCodeQueryType::QuestionTags.to_string(),
            variables: json!({ "skipCompanyTags": true}),
            query: TAGS_QUERY.to_string(),
        }
    }
}
