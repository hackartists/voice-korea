#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/panels/v2", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(primary_key, find_by_id, action = delete)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = [create], action_by_id = update)]
    pub name: String,
    #[api_model(action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(action = [create], action_by_id = update)]
    pub age: String,
    #[api_model(action = [create], action_by_id = update)]
    pub gender: String,
    #[api_model(action = [create], action_by_id = update)]
    pub region: String,
    #[api_model(action = [create], action_by_id = update)]
    pub payload: String,

    #[api_model(action = [create], query_action = list_panels)]
    pub org_id: String,
}
