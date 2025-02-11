#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use crate::attribute_v2::{GenderV2, RegionV2, SalaryV2};

use super::Question;

#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_responses)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(version = v0.1)]
    pub panel_id: i64,

    #[api_model(action = respond_answer)]
    pub proof_id: String,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub attributes: Vec<Attribute>,

    #[api_model(summary, action = respond_answer, type = JSONB)]
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

impl PartialEq<Question> for Answer {
    fn eq(&self, other: &Question) -> bool {
        match (self, other) {
            (Answer::SingleChoice { .. }, Question::SingleChoice(_)) => true,
            (Answer::MultipleChoice { .. }, Question::MultipleChoice(_)) => true,
            (Answer::ShortAnswer { .. }, Question::ShortAnswer(_)) => true,
            (Answer::Subjective { .. }, Question::Subjective(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum Attribute {
    Age(AgeV3),
    Gender(GenderV2),
    Region(RegionV2),
    Salary(SalaryV2),

    #[default]
    None,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum AgeV3 {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
    #[default]
    None,
}
