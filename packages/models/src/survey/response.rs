#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use crate::attribute_v2::{GenderV2, RegionV2, SalaryV2};

#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_response)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub proof_id: String,

    pub title: String,
    pub start_date: i64,
    pub end_date: i64,
    pub total_response_count: i64,
    pub response_count: i64,
    pub average_time: String, //TODO: 00:02:00 형태로 반환

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

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case", tag = "type")]
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
