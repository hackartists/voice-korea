// use by_axum::axum::routing::{get, post};
use models::{
    v2::{PanelV2, PanelV2Repository},
    *,
};

#[derive(Clone, Debug)]
pub struct PanelControllerV2 {
    _repo: PanelV2Repository,
}

impl PanelControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = PanelV2::get_repository(pool.clone());

        let _ctrl = PanelControllerV2 { _repo: repo };

        Ok(by_axum::axum::Router::new())
    }
}
