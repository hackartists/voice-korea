use std::sync::Arc;

use by_axum::logger::root;
use tokio::net::TcpListener;

mod common;
mod controllers {
    pub mod auth {
        pub mod v1;
    }
    pub mod verification {
        pub mod v1;
    }
    pub mod members {
        pub mod v1;
    }
    pub mod organizations {
        pub mod v1;
    }
    pub mod groups {
        pub mod v1;
    }
    pub mod attributes {
        pub mod v1;
    }
    pub mod metadatas {
        pub mod v1;
    }
    pub mod search {
        pub mod v1;
    }
    pub mod panels {
        pub mod v1;
    }
    pub mod public_opinions {
        pub mod v1;
    }
    pub mod public_surveys {
        pub mod v1;
    }
    pub mod survey { // FIXME: deprecated
        pub mod v1;
    }
}
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    let log = root();

    easy_dynamodb::init(
        log.clone(),
        option_env!("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID is required")
            .to_string(),
        option_env!("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY is required")
            .to_string(),
        option_env!("AWS_REGION")
            .unwrap_or("ap-northeast-2")
            .to_string(),
        option_env!("TABLE_NAME")
            .expect("TABLE_NAME is required")
            .to_string(),
        "id".to_string(),
        None,
        None,
    );

    let app = by_axum::new()
        .nest("/auth/v1", controllers::auth::v1::AuthControllerV1::router())
        .nest("/members/v1", controllers::members::v1::MemberControllerV1::router())
        .nest("/organizations/v1", controllers::organizations::v1::OrganizationControllerV1::router())
        .nest("/groups/v1", controllers::groups::v1::GroupControllerV1::router())
        .nest("/attributes/v1", controllers::attributes::v1::AttributeControllerV1::router())
        .nest("/metadatas/v1", controllers::metadatas::v1::MetadataControllerV1::router())
        .nest("/search/v1", controllers::search::v1::SearchControllerV1::router())
        .nest("/panels/v1", controllers::panels::v1::PanelControllerV1::router())
        .nest("/public-opinions/v1", controllers::public_opinions::v1::PublicOpinionControllerV1::router())
        .nest("/public-surveys/v1", controllers::public_surveys::v1::PublicSurveyControllerV1::router())
        .nest("/survey/v1", controllers::survey::v1::AxumState::router()); // FIXME: deprecated

    #[cfg(feature = "reload")]
    {
        use listenfd::ListenFd;
        let mut listenfd = ListenFd::from_env();
        let listener = match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true).unwrap();
                TcpListener::from_std(listener).unwrap()
            }
            None => {
                eprintln!("LISTENFD ERROR");
                return;
            }
        };
        slog::info!(
            log,
            "[AUTO-RELOAD] listening on {}",
            listener.local_addr().unwrap()
        );
        by_axum::serve(listener, app).await.unwrap();
    }
    #[cfg(not(feature = "reload"))]
    {
        let port = option_env!("PORT").unwrap_or("3000");
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .unwrap();
        slog::info!(log, "listening on {}", listener.local_addr().unwrap());
        by_axum::serve(listener, app).await.unwrap();
    }
}
