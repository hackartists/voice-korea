use by_axum::axum::routing::post;
use by_axum::axum::{Json, Router};
use by_axum::{axum::extract::State, log::root};
use models::prelude::{Organization, OrganizationMember, Role};
use models::{User, error::ApiError};
use serde::Deserialize;
use slog::o;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::utils::hash::get_hash_string;

#[derive(Deserialize, Clone, Debug)]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct SignupControllerV1 {
    log: slog::Logger,
}

impl SignupControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "SignupControllerV1"));
        let ctrl = SignupControllerV1 { log };

        Router::new()
            .route("/", post(Self::signup))
            .with_state(ctrl.clone())
    }

    pub async fn signup(
        State(ctrl): State<SignupControllerV1>,
        Json(body): Json<SignUpParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "signup"));
        slog::debug!(log, "signup {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        let auth_doc_id = verify_handler(
            Json(EmailVerifyParams {
                id: body.auth_id.clone(),
                value: body.auth_value.clone(),
            }),
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
        let _ = cli.delete(&auth_doc_id);
        let _ = cli
            .create(user.clone())
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        let org_id = SignupControllerV1::create_organization(user.id.clone(), body.clone()).await?;

        let _ = SignupControllerV1::create_member(org_id, user.id).await?; //FIXME: add to organization

        Ok(())
    }

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

    async fn create_member(org_id:String, user_id: String) -> Result<(), ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let organization_member_id = uuid::Uuid::new_v4().to_string();
        let organization_member: OrganizationMember =
            OrganizationMember::new(organization_member_id, user_id.clone(), org_id.clone(), Some(Role::Admin));
        let _ = cli
            .upsert(organization_member.clone())
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;


        match cli.upsert(organization_member.clone()).await {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Create Organization Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }
}
