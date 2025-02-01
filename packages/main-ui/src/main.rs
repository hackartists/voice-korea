#![allow(non_snake_case)]

use dioxus_logger::tracing;

use dioxus::prelude::*;
use main_ui::service::attribute_api::AttributeApi;
use main_ui::service::auth_api::AuthApi;
use main_ui::service::group_api::GroupApi;
use main_ui::service::member_api::MemberApi;
use main_ui::service::opinion_api::OpinionApi;
use main_ui::service::organization_api::OrganizationApi;
use main_ui::service::panel_api::PanelApi;
use main_ui::service::popup_service::PopupService;

use main_ui::config;
use main_ui::service::metadata_api::ResourceApi;
use main_ui::service::prev_survey_api::PrevSurveyApi;
use main_ui::service::survey_api::SurveyApi;
use main_ui::service::theme::Theme;
use main_ui::service::user_api::UserApi;

use main_ui::{
    routes::Route, service::login_service::LoginService, utils::context::use_iitp_context_provider,
};

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    #[cfg(feature = "server")]
    {
        easy_dynamodb::init(
            main_ui::utils::logger::root(),
            option_env!("AWS_ACCESS_KEY_ID")
                .expect("AWS_ACCESS_KEY_ID is required")
                .to_string(),
            option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required")
                .to_string(),
            option_env!("AWS_REGION")
                .unwrap_or("ap-northeast-2")
                .to_string(),
            option_env!("TABLE_NAME")
                .expect("TABLE_NAME is required")
                .to_string(),
            "id".to_string(),
            None,
            None,
        );
    }

    tracing::info!("starting app");
    dioxus_aws::launch(App);
}

fn App() -> Element {
    use_iitp_context_provider();
    Theme::init();
    LoginService::init();
    PopupService::init();

    OrganizationApi::init();
    MemberApi::init();
    AuthApi::init();
    UserApi::init();
    GroupApi::init();
    OpinionApi::init();
    AttributeApi::init();
    PanelApi::init();
    ResourceApi::init();
    SurveyApi::init();
    PrevSurveyApi::init();

    rsx! {
        head {
            link {
                rel: "icon",
                r#type: "image/x-icon",
                href: asset!("/public/favicon.ico"),
            }
            link { rel: "stylesheet", href: asset!("/public/main.css") }
            link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
            load_tailwindcss {}
        }
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        script { src: "https://cdn.tailwindcss.com/3.4.5" }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {}
}
