pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::OrganizationMemberResponse;

use crate::api::common::CommonQueryResponse;

use super::login_service::LoginService;

#[derive(Debug, serde::Deserialize, Clone)]
struct MyError {
    message: String,
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError {
            message: err.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrganizationApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organizations: Signal<Vec<OrganizationMemberResponse>>,
    pub selected_organization_id: Signal<String>,
}

impl OrganizationApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let srv = Self {
            endpoint: use_signal(|| {
                format!(
                    "{}",
                    option_env!("API_URL").unwrap_or("https://voice-korea-api.dev.biyard.co")
                )
            }),
            login_service,
            organizations: use_signal(|| vec![]),
            selected_organization_id: use_signal(|| "".to_string()),
        };
        use_context_provider(|| srv);
    }

    pub async fn list_organizations(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
        // FIXME: Add temporary return type for deserializable error
    ) -> std::result::Result<CommonQueryResponse<OrganizationMemberResponse>, String> {
        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let res = rest_api::get_with_query::<
            CommonQueryResponse<OrganizationMemberResponse>,
            MyError,
            _,
        >("/v1/organizations", &params)
        .await;

        // match res.clone() {
        //     Ok(v) => {
        //         tracing::debug!("this line come111 {:?}", v.items);
        //     }
        //     Err(e) => {
        //         tracing::error!("this line come222: {:?}", e.message);
        //     }
        // };

        res.map_err(|e| e.message)
    }

    pub fn set_organization(&mut self, organizations: Vec<OrganizationMemberResponse>) {
        self.organizations.set(organizations.clone());

        if organizations.len() != 0 {
            self.selected_organization_id
                .set(organizations.get(0).unwrap().organization_id.clone());
        }
    }

    pub fn get_organizations(&self) -> Vec<OrganizationMemberResponse> {
        (self.organizations)()
    }

    pub fn set_selected_organization_id(&mut self, id: String) {
        self.selected_organization_id.set(id);
    }

    pub fn get_selected_organization_id(&self) -> String {
        (self.selected_organization_id)()
    }
}
