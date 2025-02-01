pub mod verification;

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;
use verification::VerificationControllerV1;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    repo: UserRepository,
}

impl UserControllerV1 {
    pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = User::get_repository(pool.clone());

        repo.create_table().await?;

        let ctrl = UserControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_user))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_user).get(Self::list_user))
            .with_state(ctrl.clone())
            .nest(
                "/verification",
                VerificationControllerV1::route(pool.clone()).await?,
            ))
    }

    pub async fn act_user(
        State(_ctrl): State<UserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act_user {:?}", body);
        Ok(Json(User::default()))
    }

    pub async fn get_user(
        State(_ctrl): State<UserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<User>> {
        tracing::debug!("get_user {:?}", id);
        Ok(Json(User::default()))
    }

    pub async fn list_user(
        State(_ctrl): State<UserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<UserQuery>,
    ) -> Result<Json<UserGetResponse>> {
        tracing::debug!("list_user {:?}", q);

        Ok(Json(UserGetResponse::Query(QueryResponse::default())))
    }
}

// use by_axum::{
//     axum::{
//         extract::State,
//         http::header::SET_COOKIE,
//         response::Response,
//         // middleware,
//         routing::post,
//         Json,
//         Router,
//     },
//     log::root,
// };
// use models::prelude::{
//     ApiError, AuthActionRequest, EmailVerifyParams, LoginParams, ResetParams, Role, SignUpParams,
//     User,
// };
// use slog::o;

// use crate::controllers::{
//     members::v1::MemberControllerV1, organizations::v1::OrganizationControllerV1,
//     verification::v1::VerificationControllerV1,
// };
// use crate::{
//     common::CommonQueryResponse,
//     utils::{hash::get_hash_string, jwt::generate_jwt},
// };

// #[derive(Clone, Debug)]
// pub struct AuthControllerV1 {
//     log: slog::Logger,
// }

// impl AuthControllerV1 {
//     pub fn router() -> Router {
//         let log = root().new(o!("api-controller" => "AuthControllerV1"));
//         let ctrl = AuthControllerV1 { log };

//         Router::new()
//             .route("/", post(Self::auth_action))
//             .route("/login", post(Self::login))
//             .with_state(ctrl.clone())
//     }

//     pub async fn auth_action(
//         State(ctrl): State<AuthControllerV1>,
//         Json(body): Json<AuthActionRequest>,
//     ) -> Result<(), ApiError> {
//         let log = ctrl.log.new(o!("api" => "auth_action"));
//         slog::debug!(log, "auth_action {:?}", body);

//         match body {
//             AuthActionRequest::Reset(params) => Self::reset(State(ctrl), Json(params)).await,
//             AuthActionRequest::SignUp(params) => Self::signup(State(ctrl), Json(params)).await,
//         }
//     }

//     pub async fn login(
//         State(ctrl): State<AuthControllerV1>,
//         Json(body): Json<LoginParams>,
//     ) -> Result<Response<String>, ApiError> {
//         let log = ctrl.log.new(o!("api" => "login"));
//         slog::debug!(log, "login {:?}", body);
//         let email = body.email.clone();
//         let users = CommonQueryResponse::<models::User>::query(
//             &log,
//             "gsi1-index",
//             None,
//             Some(1),
//             vec![("gsi1", models::User::gsi1(body.email.clone()))],
//         )
//         .await?;

//         if users.items.len() == 0 {
//             return Err(ApiError::InvalidCredentials(email));
//         }
//         let user = users.items.first().unwrap();

//         let hashed_password = get_hash_string(body.password.as_bytes());
//         slog::debug!(
//             log,
//             "user_password: {} hashed_password: {}",
//             user.password,
//             hashed_password
//         );

//         if user.password != hashed_password {
//             return Err(ApiError::InvalidCredentials(email));
//         }

//         let jwt = generate_jwt(&user.id, &user.email)
//             .map_err(|e| ApiError::JWTGenerationFail(e.to_string()))?;

//         Ok(Response::builder()
//             .status(200)
//             .header(
//                 SET_COOKIE,
//                 format!("token={}; HttpOnly; Secure; SameSite=None; Path=/", jwt),
//             )
//             .body(jwt)
//             .map_err(|e| ApiError::ValidationError(e.to_string()))?)
//     }

//     pub async fn reset(
//         State(ctrl): State<AuthControllerV1>,
//         Json(body): Json<ResetParams>,
//     ) -> Result<(), ApiError> {
//         let log = ctrl.log.new(o!("api" => "reset"));
//         slog::debug!(log, "reset {:?}", body);
//         let cli = easy_dynamodb::get_client(&log);

//         VerificationControllerV1::verify_email(EmailVerifyParams {
//             id: body.auth_id,
//             value: body.auth_value,
//         })
//         .await?;
//         let email = body.email.clone();

//         let users = CommonQueryResponse::<models::User>::query(
//             &log,
//             "gsi1-index",
//             None,
//             Some(1),
//             vec![("gsi1", models::User::gsi1(body.email.clone()))],
//         )
//         .await?;

//         if users.items.len() == 0 {
//             return Err(ApiError::NotFound);
//         }

//         let user = match users.items.first() {
//             Some(user) => user,
//             None => return Err(ApiError::InvalidCredentials(email)),
//         };

//         let hashed_password = get_hash_string(body.password.as_bytes());
//         let _ = cli
//             .update(&user.id, vec![("password", hashed_password)])
//             .await
//             .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;
//         Ok(())
//     }

//     pub async fn signup(
//         State(ctrl): State<AuthControllerV1>,
//         Json(body): Json<SignUpParams>,
//     ) -> Result<(), ApiError> {
//         let log = ctrl.log.new(o!("api" => "signup"));
//         slog::debug!(log, "signup {:?}", body);
//         let cli = easy_dynamodb::get_client(&log);

//         let auth_doc_id = VerificationControllerV1::verify_email(EmailVerifyParams {
//             id: body.auth_id.clone(),
//             value: body.auth_value.clone(),
//         })
//         .await?;

//         let hashed_pw = get_hash_string(body.password.as_bytes());
//         let user = User::new(
//             uuid::Uuid::new_v4().to_string(),
//             body.email.clone(),
//             hashed_pw,
//         );

//         let result: Result<
//             (Option<Vec<models::User>>, Option<String>),
//             easy_dynamodb::error::DynamoException,
//         > = cli
//             .find(
//                 "gsi1-index",
//                 None,
//                 Some(1),
//                 vec![("gsi1", User::gsi1(user.email.clone()))],
//             )
//             .await;
//         match result {
//             Ok((Some(docs), _)) => {
//                 if docs.len() > 0 {
//                     return Err(ApiError::DuplicateUser);
//                 }
//             }
//             _ => (),
//         };
//         cli.delete(&auth_doc_id)
//             .await
//             .map_err(|e| ApiError::DynamoDeleteException(e.to_string()))?;
//         let _ = cli
//             .create(user.clone())
//             .await
//             .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

//         let org_id =
//             OrganizationControllerV1::create_organization(user.id.clone(), body.clone()).await?;

//         let _ = MemberControllerV1::create_member(
//             user.id,
//             org_id,
//             body.email.clone(),
//             None,
//             Some(Role::Admin),
//         )
//         .await?; //FIXME: add to organization

//         Ok(())
//     }
// }

// pub async fn find_user_id_by_email(email: String) -> Result<Option<String>, ApiError> {
//     let log = root();

//     let res: CommonQueryResponse<User> = CommonQueryResponse::query(
//         &log,
//         "gsi1-index",
//         None,
//         Some(1),
//         vec![("gsi1", User::gsi1(email.clone()))],
//     )
//     .await?;

//     if res.items.len() == 0 {
//         return Ok(None);
//     }

//     let user = res.items.first().unwrap();

//     if user.deleted_at.is_some() {
//         return Ok(None);
//     }

//     Ok(Some(user.id.clone()))
// }
