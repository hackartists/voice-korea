use by_axum::{
    axum::{
        extract::State,
        // middleware,
        routing::post,
        Json, Router,
        http::header::SET_COOKIE,
        response::Response,
    },
    log::root,
};
use slog::o;
use models::prelude::{LoginParams, ResetParams, ApiError, AuthActionRequest, EmailVerifyParams,
    Organization, OrganizationMember, Role, SignUpParams, User};

use crate::{
    utils::{
        hash::get_hash_string,
        jwt::generate_jwt,
    },
    common::CommonQueryResponse,
};
use crate::controllers::verification::v1::VerificationControllerV1;

#[derive(Clone, Debug)]
pub struct AuthControllerV1 {
    log: slog::Logger,
}

impl AuthControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "AuthControllerV1"));
        let ctrl = AuthControllerV1 { log };

        Router::new()
            .route("/", post(Self::auth_action))
            .route("/login", post(Self::login))
            .with_state(ctrl.clone())
    }

    pub async fn auth_action(
        State(ctrl): State<AuthControllerV1>,
        Json(body): Json<AuthActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "auth_action"));
        slog::debug!(log, "auth_action {:?}", body);

        match body {
            AuthActionRequest::Reset(params) => Self::reset(State(ctrl), Json(params)).await,
            AuthActionRequest::SignUp(params) => Self::signup(State(ctrl), Json(params)).await,
        }
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

    pub async fn reset(
        State(ctrl): State<AuthControllerV1>,
        Json(body): Json<ResetParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "reset"));
        slog::debug!(log, "reset {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        VerificationControllerV1::verify_email(
            EmailVerifyParams {
                id: body.auth_id,
                value: body.auth_value,
            },
        )
        .await?;
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
            return Err(ApiError::NotFound);
        }

        let user = match users.items.first() {
            Some(user) => user,
            None => return Err(ApiError::InvalidCredentials(email)),
        };

        let hashed_password = get_hash_string(body.password.as_bytes());
        let _ = cli
            .update(&user.id, vec![("password", hashed_password)])
            .await
            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;
        Ok(())
    }

    pub async fn signup(
        State(ctrl): State<AuthControllerV1>,
        Json(body): Json<SignUpParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "signup"));
        slog::debug!(log, "signup {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        let auth_doc_id = VerificationControllerV1::verify_email(
            EmailVerifyParams {
                id: body.auth_id.clone(),
                value: body.auth_value.clone(),
            },
        )
        .await?;
    
        let hashed_pw = get_hash_string(body.password.as_bytes());
        let user = User::new(
            uuid::Uuid::new_v4().to_string(),
            body.email.clone(),
            hashed_pw,
        );

        let result: Result<
            (Option<Vec<models::User>>, Option<String>),
            easy_dynamodb::error::DynamoException,
        > = cli
            .find(
                "gsi1-index",
                None,
                Some(1),
                vec![("gsi1", User::gsi1(user.email.clone()))],
            )
            .await;
        match result {
            Ok((Some(docs), _)) => {
                if docs.len() > 0 {
                    return Err(ApiError::DuplicateUser);
                }
            }
            _ => (),
        };
        cli.delete(&auth_doc_id)
            .await
            .map_err(|e| ApiError::DynamoDeleteException(e.to_string()))?;
        let _ = cli
            .create(user.clone())
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        let org_id = AuthControllerV1::create_organization(user.id.clone(), body.clone()).await?;

        let _ = AuthControllerV1::create_member(org_id, user.id).await?; //FIXME: add to organization

        Ok(())
    }

    // FIXME: move to organization controller
    async fn create_organization(user_id: String, body: SignUpParams) -> Result<String, ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let id: String = uuid::Uuid::new_v4().to_string();

        let organization: Organization =
            Organization::new(id.clone(), user_id.clone(), body.email.clone());
        let _ = cli
            .upsert(organization)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        Ok(id)
    }

    // FIXME: move to member controller
    async fn create_member(org_id:String, user_id: String) -> Result<(), ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let organization_member_id = uuid::Uuid::new_v4().to_string();
        let organization_member: OrganizationMember =
            OrganizationMember::new(organization_member_id, user_id.clone(), org_id.clone(), Some(Role::Admin));

        match cli.upsert(organization_member.clone()).await {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Create Organization Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }
}