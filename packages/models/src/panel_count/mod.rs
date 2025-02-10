#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/panel-counts/v2", table = panel_counts, iter_type=QueryResponse)]
pub struct PanelCountsV2 {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = panels)]
    pub panel_id: i64,
    #[api_model(summary, many_to_one = surveys)]
    pub panel_survey_id: i64,
    #[api_model(summary)]
    pub user_count: i64,
    #[api_model(summary)]
    pub name: String,
}

#[api_model(base = "/panel-counts-surveys/v2", table = panel_counts_surveys, iter_type=QueryResponse)]
pub struct PanelCountSurveys {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
    #[api_model(many_to_one = panel_counts)]
    pub panel_counts_id: i64,
}
