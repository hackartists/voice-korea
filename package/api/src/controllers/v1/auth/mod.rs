use by_axum::axum::Router;
use login::LoginControllerV1;
use reset::ResetControllerV1;
use signup::SignupControllerV1;

mod login;
mod reset;
mod signup;

pub fn router() -> Router {
    Router::new()
        .nest("/login", LoginControllerV1::router()?)
        .nest("/signup", SignupControllerV1::router()?)
        .nest("/reset", ResetControllerV1::router()?)
}
