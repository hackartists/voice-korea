use std::sync::Arc;

use by_axum::logger::root;
use tokio::net::TcpListener;

mod common;
mod controllers {
    pub mod auth {
        pub mod v1;
    //     pub mod login {
    //         pub mod v1;
    //     }
    //     pub mod reset {
    //         pub mod v1;
    //     }
    //     pub mod signup {
    //         pub mod v1;
    //     }
    }
    pub mod organizations {
        pub mod v1;
    }
    pub mod groups {
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
        // .nest("/auth/login/v1", controllers::auth::login::v1::LoginControllerV1::router())
        // .nest("/auth/signup/v1", controllers::auth::signup::v1::SignupControllerV1::router())
        // .nest("/auth/reset/v1", controllers::auth::reset::v1::ResetControllerV1::router())
        .nest("/auth/v1", controllers::auth::v1::AuthControllerV1::router())
        .nest("/organizations/v1", controllers::organizations::v1::OrganizationControllerV1::router())
        .nest("/groups/v1", controllers::groups::v1::GroupControllerV1::router()
    );

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
