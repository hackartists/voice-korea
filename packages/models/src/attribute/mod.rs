// FIXME: remove this model when attribute page is migrating postgre db fully
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod attribute_v2;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct CreateAttributeRequest {
    pub name: String,
    pub attribute_items: Vec<AttributeItemInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct UpdateAttributeRequest {
    pub name: String,
    pub attribute_items: Vec<AttributeItemInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeResponse {
    pub id: String,
    pub name: Option<String>,
    pub attribute: Vec<AttributeItemInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeItemInfo {
    pub id: String, //id가 ""일 경우 내부에서 즉각적인 id 추가
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AttributeActionRequest {
    Create(CreateAttributeRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AttributeByIdActionRequest {
    Delete,
    Update(UpdateAttributeRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributesResponse {
    pub id: String,
    pub name: String,
    pub attribute: Vec<String>,
}
