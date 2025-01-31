use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{common::CommonQueryResponse, middleware::auth::authorization_middleware};

use models::prelude::*;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct MetadataControllerV2 {
    log: slog::Logger,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl MetadataControllerV2 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<Router, ApiError> {
        let log = root().new(o!("controller" => "MetadataControllerV2"));
        // let repo = ResourceMetadata::get_repository(pool.clone());
        let ctrl = Self { log, pool };

        repo.create_table().await?;

        Ok(Router::new()
            // .route(
            //     "/",
            //     post(Self::create_metadata)
            //         .before(authorization_middleware)
            //         .post(Self::create_metadata),
            // )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware)))
    }
}
