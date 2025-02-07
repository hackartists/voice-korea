use by_axum::{
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
    },
};
use by_types::DatabaseConfig;
use models::*;
use reqwest::StatusCode;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct OrganizationControllerV2 {}

impl OrganizationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/:id/surveys",
                crate::controllers::survey::v2::SurveyControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:id/panels",
                crate::controllers::panels::v2::PanelControllerV2::route(pool.clone())?,
            )
            .layer(middleware::from_fn(authorize_organization)))
    }
}

pub async fn authorize_organization(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    let auth = req.extensions().get::<Option<Authorization>>();
    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.unwrap();

    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.clone().unwrap();

    let user_id = match auth {
        Authorization::Bearer { claims } => claims.sub,
        _ => {
            tracing::debug!("Authorization header is not Bearer");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if !req.uri().path().starts_with("/organizations/v2/") {
        return Ok(next.run(req).await);
    }
    //

    let org_id = req
        .uri()
        .path()
        .replace("/organizations/v2/", "")
        .split("/")
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_string();

    let conf = crate::config::get();
    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let repo = User::get_repository(pool);

    let user_id = user_id.parse::<i64>().unwrap();
    let org_id = org_id.parse::<i64>().unwrap();

    match repo
        .find_one(&UserReadAction::new().find_by_id(user_id))
        .await
    {
        Ok(user) => {
            if !user.orgs.iter().any(move |o| o.id == org_id) {
                tracing::error!("User is not member of organization");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Err(e) => {
            tracing::error!("Failed to find user: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    return Ok(next.run(req).await);
}
