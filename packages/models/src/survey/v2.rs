#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use validator::ValidationError;

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "surveys/v2", table = table-name, iter_type=QueryResponse)]
pub struct SurveyV2 {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub name: String,

    #[api_model(summary)]
    pub project_type: ProjectType,

    #[api_model(summary, action = create)]
    pub project_area: ProjectArea,

    #[api_model(summary)]
    pub status: ProjectStatus,

    #[api_model(summary, aciton = create)]
    pub started_at: i64,

    #[api_model(summary, action = create)]
    pub ended_at: i64,

    #[api_model(action = create)]
    pub description: String,
    #[api_model(summary, action = create)]
    pub quotes: i64,
    // #[api_model(action = create)]
    // pub questions: Vec<Question>,

    // #[api_model(summary, one_to_many= responses, aggregator = count)]
    // pub response_count: i64,

    // #[api_model(summary, many_to_many = attrs, foreign_table_name = attributes, foreign_primary_key = attr_id, foreign_reference_key = survey_id)]
    // pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectArea {
    #[default]
    Economy = 1,
    Society = 2,
    Environment = 3,
    Education = 4,
    Culture = 5,
    Labor = 6,
    City = 7,
    Technology = 8,
    Health = 9,
    Politics = 10,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectType {
    #[default]
    Survey = 1,
    Deliberation = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    Ready = 1,
    InProgress = 2,
    Finish = 3,
}
