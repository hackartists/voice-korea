#![allow(dead_code)]
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, RequestBuilder,
};

use ::models::error::ApiError;
pub mod create_survey_request;

use create_survey_request::*;

lazy_static! {
    static ref API_ENDPOINT: &'static str =
        option_env!("NONCE_LAB_API_ENDPOINT").expect(" \"NONCE_LAB_API_ENDPOINT\" required");
    static ref API_TOKEN: &'static str =
        option_env!("NONCE_LAB_API_TOKEN").expect(" \"NONCE_LAB_API_TOKEN\" required");
}

#[derive(Debug, Clone)]
pub struct NonceLabClient {
    base_url: &'static str,
    client: Client,
}

impl NonceLabClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        let nc = crate::config::get().nonce_lab;
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", nc.token)).unwrap(),
        );
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Client Build Failed");

        Self {
            client,
            base_url: nc.endpoint,
        }
    }

    fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client.get(format!("{}{}", self.base_url, endpoint))
    }

    fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.base_url, endpoint))
    }

    pub async fn create_survey(
        &self,
        survey_dto: NonceLabCreateSurveyRequest,
    ) -> Result<u32, ApiError> {
        let res = self
            .post("/v1/vendor/survey")
            .json(&survey_dto)
            .send()
            .await
            .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;

        let res: NonceLabCreateSurveyResponse = match res.error_for_status() {
            Ok(v) => match v.json().await {
                Ok(v) => v,
                Err(e) => return Err(ApiError::JSONSerdeError(e.to_string())),
            },
            Err(e) => {
                return Err(ApiError::ReqwestFailed(e.to_string()));
            }
        };
        Ok(res.id)
    }
}
