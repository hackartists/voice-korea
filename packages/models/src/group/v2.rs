use crate::user::User;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/organizations/:org-id/group/v2", table = groups, iter_type=QueryResponse)]
pub struct GroupV2 {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = users)]
    pub org_id: String,
    #[api_model(summary)]
    pub name: String,
    #[api_model(many_to_many = group_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = group_id, unique)]
    #[serde(default)]
    pub users: Vec<User>,
}
