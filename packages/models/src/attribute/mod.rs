// FIXME: remove this model when attribute page is migrating postgre db fully
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod attribute_v2;
pub use attribute_v2::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[deprecated]
pub struct AttributeResponse {
    pub id: String,
    pub name: Option<String>,
    pub attribute: Vec<AttributeItemInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[deprecated]
pub struct AttributeItemInfo {
    pub id: String, //id가 ""일 경우 내부에서 즉각적인 id 추가
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum Attribute {
    Age(Age),
    Gender(GenderV2),
    Region(RegionV2),
    Salary(SalaryV2),

    #[default]
    None,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum Age {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
    #[default]
    None,
}
