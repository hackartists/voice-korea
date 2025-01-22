#![allow(non_snake_case)]

use dioxus_logger::tracing::{self, Level};

use dioxus::prelude::*;
use platform::service::attribute_api::AttributeApi;
use platform::service::auth_api::AuthApi;
use platform::service::group_api::GroupApi;
use platform::service::member_api::MemberApi;
use platform::service::opinion_api::OpinionApi;
use platform::service::organization_api::OrganizationApi;
use platform::service::panel_api::PanelApi;
use platform::service::popup_service::PopupService;

use platform::service::metadata_api::ResourceApi;
use platform::service::prev_survey_api::PrevSurveyApi;
use platform::service::survey_api::SurveyApi;
use platform::service::user_api::UserApi;
use platform::{
    routes::Route, service::login_service::LoginService, utils::context::use_iitp_context_provider,
};

fn main() {
    dioxus_logger::init(match option_env!("LOG_LEVEL") {
        Some("trace") => Level::TRACE,
        Some("debug") => Level::DEBUG,
        Some("info") => Level::INFO,
        Some("warn") => Level::WARN,
        Some("error") => Level::ERROR,
        _ => Level::INFO,
    })
    .expect("failed to init logger");

    #[cfg(feature = "server")]
    {
        easy_dynamodb::init(
            platform::utils::logger::root(),
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
