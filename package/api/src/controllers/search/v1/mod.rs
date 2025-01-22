use by_axum::axum::{
    extract::{Query, State},
    routing::get, 
    Json, Router
};

use crate::common::CommonQueryResponse;

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct SearchControllerV1 {
    log: slog::Logger,
}

impl SearchControllerV1 {
    pub fn router() -> Router {
        let log = by_axum::log::root().new(slog::o!("api-controller" => "SearchControllerV1"));
        let ctrl = SearchControllerV1 { log };

        Router::new().route("/", get(Self::search_handler)).with_state(ctrl.clone())
    }

    async fn search_handler(
        State(ctrl): State<SearchControllerV1>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<SearchResult>>, ApiError> {
        let log = ctrl.log.new(slog::o!("api" => "search_handler"));
        if params.query.trim().is_empty() {
            slog::error!(log, "Query Required");
            return Err(ApiError::ValidationError("\"Query Required\"".to_string()));
        }
        Ok(Json(CommonQueryResponse {
            items: vec![SearchResult::new(
                "proof-id".to_string(),
                Some(1),
                Some(2),
                None,
                Some(Age::Specific(20)),
            )],
            bookmark: None,
        }))
    }
}
