use serde::{Deserialize, Serialize};
use crate::prelude::AttributeItem;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelAttributeItem {
    pub id: String,
    pub panel_id: String,
    pub attribute_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Panel {
    pub id: String,
    pub organization_id: String,

    pub name: String,
    pub count: i64,

    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
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
