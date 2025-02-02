#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], read_action = refresh, table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key, find_by_id)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action = [signup, login, reset], unique, read_action = get_user)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, login, reset], read_action = [get_user, find_by_email])]
    #[validate(custom(function = "validate_hex"))]
    pub password: String,

    #[api_model(many_to_many = user_orgs, foreign_table_name = organizations, foreign_primary_key = org_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub orgs: Vec<Organization>,
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

#[api_model(base = "/auth/v1/organizations", table = organizations, iter_type=QueryResponse)]
pub struct Organization {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
    #[api_model(many_to_many = user_orgs, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = org_id)]
    #[serde(default)]
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoiceKoreaClaim {
    pub email: String,
}
