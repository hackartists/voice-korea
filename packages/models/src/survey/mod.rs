mod response;
mod v2;
pub use response::*;
pub use v2::*;

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SurveyPanel {
    panel: Vec<ProofId>,
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;
pub type RegionCode = u16;
pub type ProofId = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Gender {
    Male,
    Female,
    Others,
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "male" | "남자" => Ok(Gender::Male),
            "female" | "여자" => Ok(Gender::Female),
            "others" | "기타" => Ok(Gender::Others),
            _ => Err(serde::de::Error::custom(format!("Invalid gender: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Age {
    Specific(u8),
    Range {
        #[serde(rename = "inclusiveMin")]
        inclusive_min: Option<u8>,
        #[serde(rename = "inclusiveMax")]
        inclusive_max: Option<u8>,
    },
}

pub type QuotaId = u32;
pub type QuestionId = u32;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AdminSurveyCompleteRequest {
    pub ended_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AdminSurveyCompleteResponse {
    pub total: u32,
    pub succeed: u32,
    pub failed: u32,
}
