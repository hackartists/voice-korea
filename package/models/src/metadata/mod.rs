use serde::{Deserialize, Serialize};
use crate::field::Field;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GetPutObjectUriRequest {
    pub file_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GetPutObjectUriResponse {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateMetadataRequest {
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdateMetadataRequest {
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MetadataSummary {
    pub id: String,
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataActionRequest {
    Create(CreateMetadataRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataByIdActionRequest {
    Delete,
    Update(UpdateMetadataRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicOpinion {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurvey {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataAuthority {
    #[default]
    Public,
    Private,
    Restricted,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataSource {
    #[default]
    Internal,
    External,
    Goverment,
    Company,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataPurpose {
    #[default]
    DevelopmentPolicy,
    AcademicResearch,
    PublicDiscussion,
    Education,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataType {
    #[default]
    Report,
    Statistics,
    Survey,
    Thesis,
    Presentation,
    Media,
}
