#![allow(unused_variables)]
pub use crate::organization::Role;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/invitations/v2", table = invitations, iter_type=QueryResponse)]
pub struct Invitation {
    #[api_model(summary, many_to_one = users)]
    pub user_id: String,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: String,
    #[api_model(summary, many_to_one = groups)]
    pub group_id: String,
    // pub proj_id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary)]
    #[validate(email)]
    pub email: String,
    #[api_model(summary, type = INTEGER, nullable, action = [insert])]
    pub role: Option<Role>,
}
