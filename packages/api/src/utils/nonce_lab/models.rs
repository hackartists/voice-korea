#![allow(dead_code, unused)]
use std::collections::HashMap;

use models::prelude::{
    Age, Gender, Quota, RegionCode, SalaryTier, SurveyAttribute, SurveyPanel, SurveyQuestion,
};
use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// pub struct NonceLabGetSurveyDto {
//     // pub id: u32,
//     // pub title: String,
//     // pub description: Option<String>,
//     // pub status: SurveyStatus,
//     // #[serde(rename = "startedAt")]
//     // pub created_at: String,
//     // #[serde(rename = "endedAt")]
//     // pub ended_at: String,
//     // #[serde(rename = "rewardPoints")]
//     // pub reward_points: u32,
//     // #[serde(rename = "questionCount")]
//     // pub question_count: u32,
//     // participated: bool,
//     // #[serde(rename = "estimatedMinutes")]
//     // pub estimated_minutes: u32,
//     // pub quotas: Vec<Quota>,
//     // pub questions: Vec<SurveyQuestion>,
//     #[serde(rename = "responseCountMap")]
//     pub response_count_map: Option<HashMap<u32, u64>>,
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabCreateSurveyResponse {
    pub id: u32,
    //     // pub custom_id: Option<String>,
    //     // pub status: SurveyStatus,
    //     // pub started_at: String,
    //     // pub ended_at: String,
    //     // pub title: String,
    //     // pub description: String,
    //     // pub questions: u32,
    //     // pub responses: Option<bool>,
    //     // pub estimated_minutes: u32,
    //     // pub expected_responses: u32,
    //     // pub created_at: String,
    //     // pub updated_at: String,
    //     // pub responders: Vec<String>,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NonceLabSurveyResultResponse {
//     pub quotas: Vec<NonceLabQuota>,
//     pub response_array: Vec<NonceLabSurveyResponse>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NonceLabSurveyResponse {
//     // pub id: u32,
//     pub quota_id: u32,
//     pub responded_at: String,
//     pub answers: Vec<NonceLabSurveyResultAnswer>,
// }

// // Nonce Lab Responses

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NonceLabSurveyResultAnswer {
//     // pub id: u32,
//     pub text_answer: Option<String>,
//     pub choice_answer: Option<Vec<String>>,
// }
