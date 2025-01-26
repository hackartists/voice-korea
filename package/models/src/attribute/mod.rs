#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct CreateAttributeRequest {
    pub name: String,
    pub attribute_items: Vec<AttributeItemResponse>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct UpdateAttributeRequest {
    pub name: String,
    pub attribute_items: Vec<AttributeItemResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]

pub struct Attribute {
    // attributes
    pub id: String,
    pub r#type: String,
    pub gsi1: String, // attribute#organization_id
    pub organization_id: String,

    pub name: String,

    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Attribute {
    pub fn new(organization_id: String, name: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "attribute".to_string(),
            gsi1: format!("attribute#{}", organization_id),
            organization_id: organization_id.clone(),
            name,
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
        "deleted#attribute".to_string()
    }

    pub fn get_type() -> String {
        "attribute".to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeItem {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, // attribute#item#attribute_id
    pub attribute_id: String,
    pub name: String,

    pub created_at: i64,
}

impl AttributeItem {
    pub fn new(attribute_id: String, name: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "attribute#item".to_string(),
            gsi1: format!("attribute#item#{}", attribute_id),
            attribute_id,
            name,
            created_at: now,
        }
    }

    pub fn get_gsi1(attribute_id: &str) -> String {
        format!("{}#{}", Self::get_type(), attribute_id)
    }

    pub fn get_gsi1_deleted(attribute_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), attribute_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#attribute#item".to_string()
    }

    pub fn get_type() -> String {
        "attribute#item".to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeResponse {
    pub id: String,
    pub name: Option<String>,
    pub attribute: Vec<AttributeItemResponse>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeItemResponse {
    pub id: String,
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
