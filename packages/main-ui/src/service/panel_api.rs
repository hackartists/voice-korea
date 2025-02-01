pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreatePanelRequest, PanelActionRequest, PanelByIdActionRequest, PanelResponse,
    UpdatePanelRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct PanelApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl PanelApi {
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

    pub async fn update_panel(&self, panel_id: String, req: UpdatePanelRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/panels/v1/{panel_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&PanelByIdActionRequest::Update(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn create_panel(&self, req: CreatePanelRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/panels/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&PanelActionRequest::Create(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn get_panel(&self, panel_id: String) -> Result<PanelResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/panels/v1/{panel_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panel = res.json().await?;
        Ok(panel)
    }

    pub async fn search_panel(
        &self,
        keyword: String,
    ) -> Result<CommonQueryResponse<PanelResponse>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        params.insert("keyword", keyword.to_string());

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/panels/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panels = res.json().await?;
        Ok(panels)
    }

    pub async fn remove_panel(&self, panel_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(format!("/panels/v1/{}", panel_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&PanelByIdActionRequest::Delete)
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn list_panels(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<PanelResponse>> {
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
            .get(&format!("/panels/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panels = res.json().await?;

        Ok(panels)
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
