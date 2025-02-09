use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommonQueryResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TypeField {
    #[serde(untagged)]
    N(i64),
    #[serde(untagged)]
    S(String),
    #[serde(untagged)]
    B(bool),
    #[serde(untagged)]
    V(Option<Vec<String>>),
}
