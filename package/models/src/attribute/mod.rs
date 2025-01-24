use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct CreateAttributeRequest {
    pub name: String,
    pub attribute: Vec<AttributeItem>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct UpdateAttributeRequest {
    pub name: String,
    pub attribute: Vec<AttributeItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct Attribute { // attributes
    pub id: String,
    pub r#type: String,
    pub organization_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeItem {
    pub id: Option<String>, // FIXME: if postgre is implemented, this field will be not null
    pub r#type: String,
    pub attribute_id: Option<String>, // FIXME: if postgre is implemented, this field will be not null
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