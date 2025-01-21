use by_axum::axum::routing::put;
use by_axum::axum::Json;
use by_axum::axum::{extract::State, Router};
use by_axum::log::root;
use models::{User, error::ApiError};
use serde::Deserialize;
use slog::o;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::utils::hash::get_hash_string;

#[derive(Clone, Debug)]
pub struct ResetControllerV1 {
    log: slog::Logger,
}

impl ResetControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "ResetControllerV1"));
        let ctrl = ResetControllerV1 { log };

        Router::new()
            .route("/", put(Self::reset))
            .with_state(ctrl.clone())
    }

    pub async fn reset(
        State(ctrl): State<ResetControllerV1>,
        Json(body): Json<ResetParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "reset"));
        slog::debug!(log, "reset {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        verify_handler(
            Json(EmailVerifyParams {
                id: body.auth_id,
                value: body.auth_value,
            }),
        )
        .await?;
        let email = body.email.clone();

        let result: Result<
            (Option<Vec<User>>, Option<String>),
            easy_dynamodb::error::DynamoException,
        > = cli
            .find(
                "gsi1-index",
                None,
                Some(1),
                vec![("gsi1", User::gsi1(body.email))],
            )
            .await;

        let (docs, _) = match result {
            Ok((Some(docs), Some(_))) => (docs, ()),
            _ => return Err(ApiError::InvalidCredentials(email)),
        };
        let user = match docs.first() {
            Some(user) => user,
            None => return Err(ApiError::InvalidCredentials(email)),
        };
        let hashed_password = get_hash_string(body.password.as_bytes());
        let _ = cli
            .update(&user.id, vec![("password", hashed_password)])
            .await;
        Ok(())
    }
}
