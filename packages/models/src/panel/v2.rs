#![allow(unused_variables)]
use crate::attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2};
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/organizations/v2/:org-id/panels", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
    pub id: String,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update, unique)]
    pub name: String,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "age")]
    pub age: AgeV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "gender")]
    pub gender: GenderV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "region")]
    pub region: RegionV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "salary")]
    pub salary: SalaryV2,

    #[api_model(summary, queryable, many_to_one = organizations)]
    pub org_id: String,
}
