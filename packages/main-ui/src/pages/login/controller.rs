#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    service::{
        login_service::use_login_service,
        organization_api::OrganizationApi,
        user_api::{LoginRequest, UserApi},
    },
    utils::hash::get_hash_string,
};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    email: Signal<String>,
    password: Signal<String>,
    not_matched_error: Signal<bool>,
    not_exists_error: Signal<bool>,
    login_failed_error: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
            not_matched_error: use_signal(|| false),
            not_exists_error: use_signal(|| false),
            login_failed_error: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_not_matched_error(&self) -> bool {
        (self.not_matched_error)()
    }

    pub fn get_exists_error(&self) -> bool {
        (self.not_exists_error)()
    }

    pub fn get_login_failed_error(&self) -> bool {
        (self.login_failed_error)()
    }

    pub fn get_email(&self) -> String {
        (self.email)()
    }

    pub fn get_password(&self) -> String {
        (self.password)()
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_password(&mut self, password: String) {
        self.password.set(password);
    }

    pub async fn login_clicked(&mut self, lang: Language) {
        let user_api: UserApi = use_context();
        let mut login_service = use_login_service();
        let mut api: OrganizationApi = use_context();
        let navigator = use_navigator();
        let res = user_api
            .login_user(LoginRequest {
                email: self.get_email(),
                password: get_hash_string(self.get_password().as_bytes()),
            })
            .await;

        match res {
            Ok(token) => {
                login_service.setup(self.get_email(), token).await;
                let organizations = api.list_organizations(Some(100), None).await;
                let items = organizations.unwrap_or_default().items;
                api.set_organization(items);
                navigator.push(Route::DashboardPage { lang });
            }
            Err(e) => match e {
                ServerFnError::ServerError(v) => {
                    if v.contains("Wrong User") {
                        self.not_matched_error.set(true);
                    } else {
                        self.login_failed_error.set(true);
                    }
                }
                _ => {}
            },
        }
    }
}
