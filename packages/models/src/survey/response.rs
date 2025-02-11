#![allow(unused)]
use crate::{Attribute, Result};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_response)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub proof_id: String,

    #[api_model(summary, type = JSONB)]
    pub attributes: Vec<Attribute>,

    #[api_model(summary, type = JSONB)]
    pub answers: Vec<Answer>,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice { answer: i32 },
    MultipleChoice { answer: Vec<i32> },
    ShortAnswer { answer: String },
    Subjective { answer: String },
}
