pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::SurveyResultDocument;

use crate::utils::api::ReqwestClient;

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct PrevSurveyApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl PrevSurveyApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let organization_service: OrganizationApi = use_context();
        let srv = Self {
            endpoint: use_signal(|| {
                format!(
                    "{}",
                    option_env!("API_URL").unwrap_or("https://voice-korea-api.dev.biyard.co")
                )
            }),
            login_service,
            organization_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn get_survey_result(&self, survey_id: String) -> Result<SurveyResultDocument> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/survey/v1/{survey_id}/result"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let survey = res.json().await?;

        Ok(survey)
    }

    pub async fn create_survey(
        &self,
        survey_id: String,
    ) -> Result<models::prelude::ProgressSurveyResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/survey/v1/{survey_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let survey = res.json().await?;
        Ok(survey)
    }

    pub async fn list_surveys(
        &self,
        size: Option<i32>,
        bookmark: Option<String>,
    ) -> Result<models::prelude::ListSurveyResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/survey/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let surveys = res.json().await?;
        Ok(surveys)
    }

    pub async fn upsert_survey_draft(
        &self,
        req: models::prelude::UpsertSurveyDraftRequest,
    ) -> Result<String> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/survey/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&req)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let survey_id = res.json().await?;
        Ok(survey_id)
    }

    pub async fn get_survey(&self, survey_id: String) -> Result<models::prelude::Survey> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/survey/v1/{survey_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let survey = res.json().await?;

        Ok(survey)
    }

    pub fn get_organization_id(&self) -> String {
        let id = self.organization_service.get_selected_organization_id();
        id
    }

    pub fn get_token(&self) -> String {
        let cookie = if cfg!(feature = "web") {
            self.login_service
                .get_cookie_value()
                .unwrap_or_else(|| "".to_string())
        } else {
            "".to_string()
        };

        let token = cookie.replace('"', "");
        let format_cookie = format!("token={token}");
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        token
    }
}
