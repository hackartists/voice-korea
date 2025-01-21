use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Extension, Json, Router,
        http::header::SET_COOKIE,
        response::Response,
    },
    log::root,
};
use slog::o;
use models::prelude::{LoginParams, ResetParams, ApiError};
use crate::{
    utils::{
        hash::get_hash_string,
        jwt::generate_jwt,
    },
    common::CommonQueryResponse,
};

#[derive(Clone, Debug)]
pub struct AuthControllerV1 {
    log: slog::Logger,
}

impl AuthControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "AuthControllerV1"));
        let ctrl = AuthControllerV1 { log };

        Router::new()
            .route("/login", post(Self::login))
            // .route("/signup", post(Self::signup))
            // .route("/reset", post(Self::reset))
            .with_state(ctrl.clone())
    }

    pub async fn login(
        State(ctrl): State<AuthControllerV1>,
        Json(body): Json<LoginParams>,
    ) -> Result<Response<String>, ApiError> {
        let log = ctrl.log.new(o!("api" => "login"));
        slog::debug!(log, "login {:?}", body);
        let email = body.email.clone();
        let users = CommonQueryResponse::<models::User>::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", models::User::gsi1(body.email.clone()))],
        )
        .await?;

        if users.items.len() == 0 {
            return Err(ApiError::InvalidCredentials(email));
        }
        let user = users.items.first().unwrap();

        let hashed_password = get_hash_string(body.password.as_bytes());
        slog::debug!(
            log,
            "user_password: {} hashed_password: {}",
            user.password,
            hashed_password
        );

        if user.password != hashed_password {
            return Err(ApiError::InvalidCredentials(email));
        }

        let jwt = generate_jwt(&user.id, &user.email)
            .map_err(|e| ApiError::JWTGenerationFail(e.to_string()))?;

        Ok(Response::builder()
            .status(200)
            .header(
                SET_COOKIE,
                format!("token={}; HttpOnly; Secure; SameSite=None; Path=/", jwt),
            )
            .body(jwt)
            .map_err(|e| ApiError::ValidationError(e.to_string()))?)
    }

    // pub async fn reset(
    //     State(ctrl): State<AuthControllerV1>,
    //     Json(body): Json<ResetParams>,
    // ) -> Result<(), ApiError> {
    //     let log = ctrl.log.new(o!("api" => "reset"));
    //     slog::debug!(log, "reset {:?}", body);
    //     let cli = easy_dynamodb::get_client(&log);

    //     verify_handler(
    //         Json(EmailVerifyParams {
    //             id: body.auth_id,
    //             value: body.auth_value,
    //         }),
    //     )
    //     .await?;
    //     let email = body.email.clone();

    //     let result: Result<
    //         (Option<Vec<User>>, Option<String>),
    //         easy_dynamodb::error::DynamoException,
    //     > = cli
    //         .find(
    //             "gsi1-index",
    //             None,
    //             Some(1),
    //             vec![("gsi1", User::gsi1(body.email))],
    //         )
    //         .await;

    //     let (docs, _) = match result {
    //         Ok((Some(docs), Some(_))) => (docs, ()),
    //         _ => return Err(ApiError::InvalidCredentials(email)),
    //     };
    //     let user = match docs.first() {
    //         Some(user) => user,
    //         None => return Err(ApiError::InvalidCredentials(email)),
    //     };
    //     let hashed_password = get_hash_string(body.password.as_bytes());
    //     let _ = cli
    //         .update(&user.id, vec![("password", hashed_password)])
    //         .await;
    //     Ok(())
    // }
}