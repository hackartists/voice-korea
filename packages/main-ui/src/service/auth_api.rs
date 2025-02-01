pub type Result<T> = std::result::Result<T, ServerFnError>;

use dioxus::prelude::*;
use models::prelude::{EmailSendParams, EmailVerifyParams, StringJson, VerificationActionRequest};
use serde::{Deserialize, Serialize};

use crate::utils::api::ReqwestClient;

#[derive(Debug, Clone, Copy)]
pub struct AuthApi {
    pub endpoint: Signal<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SendNotificationParams {
    pub email: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyAuthenticationParams {
    pub id: String,
    pub value: String,
}

impl AuthApi {
    pub fn init() {
        let srv = Self {
            endpoint: use_signal(|| crate::config::get().api_url.to_string()),
        };
        use_context_provider(|| srv);
    }

    pub async fn send_notification(&self, req: SendNotificationParams) -> Result<String> {
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/verification/v1"))
            .json(&VerificationActionRequest::SendEmail(EmailSendParams {
                email: req.email,
            }))
            .send()
            .await?;

        let res = res.error_for_status()?;
        let json: StringJson = res.json().await?;

        Ok(json.value)
    }

    pub async fn verify_authentication(&self, req: VerifyAuthenticationParams) -> Result<String> {
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/verification/v1"))
            .json(&VerificationActionRequest::VerifyEmail(EmailVerifyParams {
                id: req.id,
                value: req.value,
            }))
            .send()
            .await?;

        let res = res.error_for_status()?;
        let json: StringJson = res.json().await?;

        Ok(json.value)
    }
}
