use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/auth/v1", action = [signup(code = String)], table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action = signup)]
    pub email: String,
    #[api_model(action = signup)]
    pub password: String,
}

const VERIFICATION_EXIPIRED_TIME: i64 = 60 * 5;
// impl User {
//     pub fn new(id: String, email: String, hashed_pw: String) -> Self {
//         Self {
//             id,
//             email,
//             password: hashed_pw,
//             created_at: Utc::now().timestamp(),
//             updated_at: None,
//         }
//     }
//     pub fn gsi1(email: String) -> String {
//         format!("user#{}", email)
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCode {}

#[api_model(base = "/auth/v1/verification", table = verifications, iter_type=QueryResponse)]
pub struct Verification {
    #[api_model(primary_key)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(action = send_verification_code, unique)]
    pub email: String,
    pub value: String,
    pub expired_at: i64,
    pub attemp_count: i32,
}

impl Verification {
    pub fn new(id: String, email: String, random_value: String) -> Self {
        Self {
            id,
            email,
            value: random_value,
            created_at: Utc::now().timestamp(),
            expired_at: Utc::now().timestamp() + VERIFICATION_EXIPIRED_TIME,
            attemp_count: 1,
        }
    }
}
