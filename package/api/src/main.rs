use std::sync::Arc;

use by_axum::logger::root;
use tokio::net::TcpListener;

mod common;
mod controllers;
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
        .nest("/", controllers::router());

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
