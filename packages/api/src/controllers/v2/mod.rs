use models::*;
use surveys::_id::responses::SurveyResponseController;

pub mod surveys {
    pub mod _id {
        pub mod responses;
    }
}

#[derive(Clone, Debug)]
pub struct Version2Controller {}

impl Version2Controller {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new().nest(
            "/surveys/:survey-id/responses",
            SurveyResponseController::route(pool)?,
        ))
    }
}
