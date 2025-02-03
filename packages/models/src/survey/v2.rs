#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use chrono::{TimeZone, Utc};
use dioxus_translate::Translate;
use validator::ValidationError;

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "/surveys/v2", table = surveys, iter_type=QueryResponse)]
pub struct SurveyV2 {
    #[api_model(summary, primary_key, read_action = find_by_id)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub name: String,

    #[api_model(summary, type = INTEGER)]
    pub project_type: ProjectType,

    #[api_model(summary, action = create, type = INTEGER)]
    pub project_area: ProjectArea,

    #[api_model(summary)]
    pub status: ProjectStatus,

    #[api_model(summary, action = create)]
    pub started_at: i64,

    #[api_model(summary, action = create)]
    pub ended_at: i64,

    #[api_model(action = create)]
    pub description: String,
    #[api_model(summary, action = create)]
    pub quotes: i64,

    #[api_model(summary, action = create, many_to_one = organizations)]
    pub org_id: String,
    #[api_model(action = create, type = JSONB, version = v0.1)]
    pub questions: Vec<Question>,
    // #[api_model(summary, one_to_many= responses, aggregator = count)]
    // pub response_count: i64,

    // #[api_model(summary, many_to_many = attrs, foreign_table_name = attributes, foreign_primary_key = attr_id, foreign_reference_key = survey_id)]
    // pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answerType")]
pub enum Question {
    SingleChoice(ChoiceQuestion),
    MultipleChoice(ChoiceQuestion),
    ShortAnswer(SubjectiveQuestion),
    Subjective(SubjectiveQuestion),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectArea {
    #[default]
    #[translate(ko = "경제")]
    Economy = 1,
    #[translate(ko = "사회")]
    Society = 2,
    #[translate(ko = "환경")]
    Environment = 3,
    #[translate(ko = "교육")]
    Education = 4,
    #[translate(ko = "문화")]
    Culture = 5,
    #[translate(ko = "노동")]
    Labor = 6,
    #[translate(ko = "도시")]
    City = 7,
    #[translate(ko = "기술")]
    Technology = 8,
    #[translate(ko = "보건")]
    Health = 9,
    #[translate(ko = "정치")]
    Politics = 10,
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectType {
    #[default]
    #[translate(ko = "설문조사")]
    Survey = 1,
    #[translate(ko = "공론조사")]
    Deliberation = 2,
}

// FIXME: rename to ProjectStatus after finishing migration from public_opinion.
#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    #[translate(ko = "준비")]
    Ready = 1,
    #[translate(ko = "진행")]
    InProgress = 2,
    #[translate(ko = "마감")]
    Finish = 3,
}

impl SurveyV2Summary {
    pub fn start_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.started_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn end_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.ended_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn period(&self) -> String {
        format!("{} ~ {}", self.start_date(), self.end_date())
    }

    pub fn response_rate(&self) -> String {
        // TODO: implement real logic for calculation of response rate.
        let responses = 0;

        format!(
            "{}% ({}/{})",
            responses / self.quotes * 100,
            responses,
            self.quotes
        )
    }
}
