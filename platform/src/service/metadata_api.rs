pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreateMetadataRequest, GetPutObjectUriRequest, GetPutObjectUriResponse, MetadataActionRequest,
    MetadataByIdActionRequest, MetadataSummary, UpdateMetadataRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct ResourceApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl ResourceApi {
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

    pub async fn create_metadata(&self, req: CreateMetadataRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/metadatas/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&MetadataActionRequest::Create(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn update_metadata(
        &self,
        metadata_id: String,
        req: UpdateMetadataRequest,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/metadatas/v1/{metadata_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&MetadataByIdActionRequest::Update(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn list_metadata(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<MetadataSummary>> {
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
            .get(&format!("/metadatas/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let metadatas = res.json().await?;
        Ok(metadatas)
    }

    pub async fn upload_metadata(
        &self,
        req: GetPutObjectUriRequest,
    ) -> Result<GetPutObjectUriResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(format!("/metadatas/v1/upload").as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&req)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let metadata = res.json().await?;
        Ok(metadata)
    }

    pub async fn remove_metadata(&self, metadata_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(format!("/metadatas/v1/{}", metadata_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&MetadataByIdActionRequest::Delete)
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn get_metadata(&self, metadata_id: String) -> Result<MetadataSummary> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/metadatas/v1/{metadata_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let metadata = res.json().await?;
        Ok(metadata)
    }

    pub async fn search_metadatas(
        &self,
        keyword: String,
    ) -> Result<CommonQueryResponse<MetadataSummary>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        params.insert("keyword", keyword.to_string());

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/metadatas/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let metadatas = res.json().await?;
        Ok(metadatas)
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
