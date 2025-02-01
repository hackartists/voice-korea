pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreatePublicSurveyRequest, PublicSurveyResponse, PublicSurveySummary, SurveyActionRequest,
    SurveyByIdActionRequest, UpdatePublicSurveyRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct SurveyApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl SurveyApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let organization_service: OrganizationApi = use_context();
        let srv = Self {
            endpoint: use_signal(|| crate::config::get().api_url.to_string()),
            login_service,
            organization_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn update_survey(
        &self,
        survey_id: String,
        req: UpdatePublicSurveyRequest,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/public-surveys/v1/{survey_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&SurveyByIdActionRequest::Update(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn create_survey(&self, req: CreatePublicSurveyRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/public-surveys/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&SurveyActionRequest::Create(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn get_survey(&self, survey_id: String) -> Result<PublicSurveyResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/public-surveys/v1/{survey_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let survey = res.json().await?;
        Ok(survey)
    }

    pub async fn remove_survey(&self, survey_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(format!("/public-surveys/v1/{}", survey_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&SurveyByIdActionRequest::Delete)
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn search_surveys(
        &self,
        keyword: String,
    ) -> Result<CommonQueryResponse<PublicSurveySummary>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        params.insert("keyword", keyword.to_string());

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/public-surveys/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let surveys = res.json().await?;
        Ok(surveys)
    }

    pub async fn list_surveys(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<PublicSurveySummary>> {
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
            .get(&format!("/public-surveys/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let surveys = res.json().await?;
        Ok(surveys)
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
