use by_axum::axum::{
    // routing::post, 
    Router
};
use by_axum::routing::post;

pub mod email;

pub fn router() -> Result<Router> {
    Ok(Router::new()
        .route("/email/send", post(email::send_handler))
        .route("/email/verify", post(email::verify_handler)))
}
