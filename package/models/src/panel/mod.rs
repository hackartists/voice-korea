use serde::{Deserialize, Serialize};
use crate::prelude::PanelAttributeDetailInfo;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelAttributeInfo {
    pub id: Option<String>,
    pub name: String,
    pub attribute: Vec<PanelAttributeDetailInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelSummary {
    pub id: String,
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum PanelActionRequest {
    Create(CreatePanelRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum PanelByIdActionRequest {
    Delete,
    Update(UpdatePanelRequest),
}
