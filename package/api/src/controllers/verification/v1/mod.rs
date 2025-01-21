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
use crate::utils::email::send_email;
use models::{
    AuthDocument, 
    error::ApiError,
    EmailSendParams,
};

#[derive(Clone, Debug)]
pub struct VerificationControllerV1 {
    log: slog::Logger,
}

impl VerificationControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "VerificationControllerV1"));
        let ctrl = VerificationControllerV1 { log };

        Router::new()
            .route("/", post(Self::act_verification))
            .with_state(ctrl.clone())
    }

    pub async fn act_verification(
        State(ctrl): State<VerificationControllerV1>,
        Json(body): Path<VerificationActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_verification"));
        slog::debug!(log, "act_verification {:?}", params);
        
    }

    pub async fn email_send(
        Json(body): Json<EmailSendParams>,
    ) -> Result<String, ApiError> {
        //TODO: If Email send failed, remove Document
        //TODO: Add request limit
        let log = root();
        slog::debug!(log, "send_handler {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        let random_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
        let doc = models::AuthDocument::new(
            uuid::Uuid::new_v4().to_string(),
            body.email.clone(),
            random_string.clone(),
        );
        let doc_id = doc.id.clone();
        match cli.create(doc).await {
            Ok(_) => {
                match send_email(
                    body.email,
                    Content::builder()
                        .data("인증번호 6자리입니다. 확인 후 3분 이내에 입력해주세요.")
                        .build()
                        .unwrap(),
                    Content::builder()
                        .data(format!("인증번호: {:?}", random_string))
                        .build()
                        .unwrap(),
                )
                .await
                {
                    Ok(v) => {
                        slog::debug!(log, "Email Send {}", v);
                        Ok(doc_id)
                    }
                    Err(e) => Err(ApiError::SESServiceError(e.to_string())),
                }
            }
            Err(e) => Err(ApiError::DynamoCreateException(e.to_string())),
        }
    }

    pub async fn verify_email(
        Json(body): Json<EmailVerifyParams>,
    ) -> Result<String, ApiError> {
        let log = root();
        slog::debug!(log, "verify_email {:?}", body);
        let cli = easy_dynamodb::get_client(&log);

        let result: Result<Option<AuthDocument>, easy_dynamodb::error::DynamoException> =
            cli.get(&body.id).await;
        let auth = match result {
            Ok(Some(v)) => v,
            Ok(None) => return Err(ApiError::AuthKeyNotMatch(body.id)),
            Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
        };
        let auth_doc_id = auth.id.clone();

        if auth.value != body.value || auth.expired_at < Utc::now().timestamp() {
            return Err(ApiError::AuthKeyNotMatch(body.id));
        }

        Ok(auth_doc_id)
    }
}