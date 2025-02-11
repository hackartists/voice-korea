#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/organizations/v2/:org-id/surveys/:survey-id/responses", read_action = download_excel, database = skip)]
pub struct SurveyResponseExcel {
    pub url: String,
}
