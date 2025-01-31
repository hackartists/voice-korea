use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/auth/v1", read_action = user_info, table = users, iter_type=QueryResponse)]
pub struct User {
    pub id: String,
    pub gsi1: String, // id#email
    pub email: String,
    pub password: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
}

const VERIFICATION_EXIPIRED_TIME: i64 = 60 * 5;
impl User {
    pub fn new(id: String, email: String, hashed_pw: String) -> Self {
        Self {
            id,
            gsi1: User::gsi1(email.clone()),
            email,
            password: hashed_pw,
            created_at: Utc::now().timestamp(),
            updated_at: None,
            deleted_at: None,
        }
    }
    pub fn gsi1(email: String) -> String {
        format!("user#{}", email)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthDocument {
    pub r#type: String,
    pub id: String,
    pub email: String,
    pub value: String,
    pub created_at: i64,
    pub expired_at: i64,
    pub attemp_count: u8,
}

impl AuthDocument {
    pub fn new(id: String, email: String, random_value: String) -> Self {
        Self {
            r#type: "verify".to_string(),
            id,
            email,
            value: random_value,
            created_at: Utc::now().timestamp(),
            expired_at: Utc::now().timestamp() + VERIFICATION_EXIPIRED_TIME,
            attemp_count: 1,
        }
    }
}
