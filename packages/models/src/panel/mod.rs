use crate::attribute::AttributeResponse;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<AttributeResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdatePanelRequest {
    pub name: String,
    pub count: i64,
    pub attribute: Vec<AttributeResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelAttributeItem {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, // panel_attribute#panel_id
    pub gsi2: String, // panel_attribute#panel_id#attribute_id : unique
    pub panel_id: String,
    pub attribute_id: String,

    pub created_at: i64,
}

impl PanelAttributeItem {
    pub fn new(panel_id: String, attribute_id: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "panel_attribute".to_string(),
            gsi1: format!("panel_attribute#{}", panel_id),
            gsi2: format!("panel_attribute#{}#{}", panel_id, attribute_id),
            panel_id,
            attribute_id,
            created_at: now,
        }
    }

    pub fn get_gsi1(panel_id: &str) -> String {
        format!("{}#{}", Self::get_type(), panel_id)
    }

    pub fn get_gsi2(panel_id: &str, attribute_id: &str) -> String {
        format!("{}#{}#{}", Self::get_type(), panel_id, attribute_id)
    }

    pub fn get_type() -> String {
        "panel_attribute".to_string()
    }
}

#[derive(validator::Validate)]
pub struct PanelV2 {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,

    pub name: String,
    pub quotes: i64,

    pub age: String,
    pub salary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Panel {
    pub id: String,
    pub r#type: String,
    pub organization_id: String,
    pub gsi1: String, // panel#organization_id

    pub name: String,
    pub count: i64,

    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Panel {
    pub fn new(organization_id: String, name: String, count: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "panel".to_string(),
            organization_id: organization_id.clone(),
            gsi1: format!("panel#{}", organization_id),
            name,
            count,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn get_gsi1(organization_id: &str) -> String {
        format!("{}#{}", Self::get_type(), organization_id)
    }

    pub fn get_gsi1_deleted(organization_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), organization_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#panel".to_string()
    }

    pub fn get_type() -> String {
        "panel".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelResponse {
    pub id: String,
    pub name: Option<String>,
    pub count: Option<i64>,
    pub attribute: Vec<AttributeResponse>,
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
