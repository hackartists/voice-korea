#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action = [signup, login, reset], unique, read_action = get_user)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, login, reset], read_action = get_user)]
    #[validate(custom(function = "validate_hex"))]
    pub password: String,
}

#[derive(validator::Validate)]
#[api_model(base = "/auth/v1/verification", table = verifications, iter_type=QueryResponse)]
pub struct Verification {
    #[api_model(primary_key)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(action = send_verification_code, read_action = get_verification_code)]
    #[validate(email)]
    pub email: String,
    #[api_model(read_action = get_verification_code)]
    pub value: String,
    pub expired_at: i64,
    pub attemp_count: i32,
}

fn validate_hex(value: &str) -> std::result::Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    if re.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_hex"))
    }
}
