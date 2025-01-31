use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    pub survey_id: String,
    pub title: String,
    pub question: String,
    pub options: Option<Vec<String>>,
    pub created_at: u64,
    pub updated_at: u64,
    pub gsi1: String,
    pub gsi2: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Text(Option<String>),
    LongText(Option<String>),
    SingleChoice {
        // question is a description of the question
        question: Option<String>,
        options: Vec<String>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionAnswer {
    Text(String),
    LongText(String),
    SingleChoice(usize),
}
