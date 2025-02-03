use crate::Field;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
#[cfg(feature = "server")]
use schemars::JsonSchema;
#[derive(validator::Validate)]
#[api_model(base = "/resource/v1", table = resources, iter_type=QueryResponse)]
pub struct Resource {
    //FIXME: When add "action_by_id = delete", Error occured.
    /*
        error[E0415]: identifier `id` is bound more than once in this parameter list
        --> packages/models/src/metadata/v2.rs:14:9
    14 |     pub id: String,
       |         ^^ used as parameter more than once
        */
    #[api_model(summary, primary_key, read_action = find_by_id )]
    #[allow(unused)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,

    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub resource_type: Option<ResourceType>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub field: Option<Field>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub usage_purpose: Option<UsagePurpose>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub source: Option<Source>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub access_level: Option<AccessLevel>,

    #[api_model(action = create, query_action = list_resources)]
    #[allow(unused)]
    pub org_id: String,
    // TODO: After Implement Deliberation Table
    // #[api_model(many_to_many = resource_delierations, foreign_table_name = delierations, foreign_primary_key = delieration_id, foreign_reference_key = resource_id)]
    // pub deliberations: Option<Vec<Deliberation>>,

    // TODO: After Implement Survey Table
    // #[api_model(many_to_many = resource_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = resource_id)]
    // pub surveys: Option<Vec<Survey>>,

    // FIXME: "one_to_many" is not supported yet
    // #[api_model(one_to_many = metadatas, foreign_key = resource_id)]
    // #[serde(default)]
    // pub files: Vec<Metadata>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ResourceType {
    #[default]
    Report = 1,
    Statistics = 2,
    Survey = 3,
    Thesis = 4,
    Presentation = 5,
    Media = 6,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum UsagePurpose {
    #[default]
    PolicyDevelopment = 1,
    AcademicResearch = 2,
    PublicDebate = 3,
    EducationalMaterial = 4,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Source {
    #[default]
    Internal = 1,
    External = 2,
    Government = 3,
    Company = 4,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AccessLevel {
    #[default]
    Public = 1,
    Restricted = 2,
    Confidential = 3,
}

// TODO: After Implement "One-to-Many" Relationship

// #[api_model(base = "/resource/v1/metadata", table = metadatas, iter_type=QueryResponse)]
// pub struct Metadata {
//     #[api_model(primary_key, read_action = find_by_id)]
//     pub id: String,
//     #[api_model(auto = insert)]
//     pub created_at: i64,
//     #[api_model(auto = [insert, update])]
//     pub updated_at: i64,
//     #[api_model(action = create)]
//     pub url: String,
//     #[api_model(action = create)]
//     pub format: Format,
//     // FIXME: "one_to_many" is not supported yet
//     // #[api_model(many_to_one = resources)]
//     // pub resource_id: String,
// }

// #[derive(Debug, Default, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, ApiModel)]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub enum Format {
//     #[default]
//     PDF = 1,
//     Excel = 2,
//     Word = 3,
//     Media = 4,
// }
