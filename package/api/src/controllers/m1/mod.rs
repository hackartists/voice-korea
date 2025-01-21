use by_axum::axum::{middleware, Router};

use crate::middleware::auth::admin_authorization_middleware;

mod survey;

pub fn router() -> Router {
    Router::new()
        .nest("/survey", survey::router())
        .layer(middleware::from_fn(admin_authorization_middleware))
}
