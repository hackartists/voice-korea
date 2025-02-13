#![allow(unused_variables)]
use crate::organization::member::Role;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/organizations/v2/:org-id/invite", table = invitation, action = [invite(email = String)], iter_type=QueryResponse)]
pub struct Invitation {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, many_to_one=users, unique)]
    pub user_id: i64,
    #[api_model(summary, many_to_one=organizations, unique)]
    pub org_id: i64,
    #[api_model(summary, many_to_one=groups, action = [invite])]
    pub group_id: Option<i64>,
    // #[api_model(summary, many_to_one=projects)]
    // pub project_id: i64,
    #[api_model(summary)]
    #[validate(email)]
    pub email: String,
    #[api_model(summary, action = [invite])]
    pub name: Option<String>,
    #[api_model(summary, type = INTEGER, action = [invite])]
    pub role: Option<Role>,
}
