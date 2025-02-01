use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::middleware,
};
use by_types::DatabaseConfig;
use models::ApiError;
use sqlx::postgres::PgPoolOptions;
// use by_types::DatabaseConfig;
// use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod common;
mod controllers {
    pub mod auth {
        pub mod v1;
    }
    // pub mod members {
    //     pub mod v1;
    // }
    // pub mod organizations {
    //     pub mod v1;
    // }
    // pub mod groups {
    //     pub mod v1;
    // }
    // pub mod attributes {
    //     pub mod v1;
    // }
    // pub mod metadatas {
    //     pub mod v1;
    //     // pub mod v2;
    // }
    // pub mod search {
    //     pub mod v1;
    // }
    // pub mod panels {
    //     pub mod v1;
    // }
    // pub mod public_opinions {
    //     pub mod v1;
    // }
    // pub mod public_surveys {
    //     pub mod v1;
    // }
    // pub mod survey {
    //     // FIXME: deprecated
    //     pub mod m1;
    //     pub mod v1;
    // }
}
pub mod config;
// mod middleware;
mod utils;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let conf = config::get();
    set_auth_config(conf.auth.clone());

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let app = by_axum::new()
        .nest(
            "/auth/v1",
            controllers::auth::v1::UserControllerV1::route(pool.clone()).await?,
        )
        .layer(middleware::from_fn(authorization_middleware));
    // .nest(
    //     "/members/v1",
    //     controllers::members::v1::MemberControllerV1::router(),
    // )
    // .nest(
    //     "/organizations/v1",
    //     controllers::organizations::v1::OrganizationControllerV1::router(),
    // )
    // .nest(
    //     "/groups/v1",
    //     controllers::groups::v1::GroupControllerV1::router(),
    // )
    // .nest(
    //     "/attributes/v1",
    //     controllers::attributes::v1::AttributeControllerV1::router(),
    // )
    // .nest(
    //     "/metadatas/v1",
    //     controllers::metadatas::v1::MetadataControllerV1::router(),
    // )
    // .nest(
    //     "/metadatas/v2",
    //     controllers::metadatas::v2::MetadataControllerV2::route(pool.clone())
    //         .await
    //         .unwrap(),
    // )
    // .nest(
    //     "/search/v1",
    //     controllers::search::v1::SearchControllerV1::router(),
    // )
    // .nest(
    //     "/panels/v1",
    //     controllers::panels::v1::PanelControllerV1::router(),
    // )
    // .nest(
    //     "/public-opinions/v1",
    //     controllers::public_opinions::v1::PublicOpinionControllerV1::router(),
    // )
    // .nest(
    //     "/public-surveys/v1",
    //     controllers::public_surveys::v1::PublicSurveyControllerV1::router(),
    // )
    // .nest("/survey/v1", controllers::survey::v1::AxumState::router()) // FIXME: deprecated
    // .nest("/survey/m1", controllers::survey::m1::AxumState::router()); // FIXME: deprecated

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
