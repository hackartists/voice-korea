use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{QueryResponse, SurveyV2, SurveyV2Query, SurveyV2Summary};

use crate::config;
use crate::pages::surveys::page::RemoveSurveyModal;
use crate::service::login_service::LoginService;
use crate::service::popup_service::PopupService;

use super::i18n::SurveyTranslate;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    pub surveys: Resource<QueryResponse<SurveyV2Summary>>,
    popup_service: PopupService,
    page: Signal<usize>,
    pub size: usize,
    translate: Signal<SurveyTranslate>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let translate: SurveyTranslate = translate(&lang);
        let page = use_signal(|| 1);
        let size = 10;
        let user: LoginService = use_context();

        // FIXME: it causes screen flickering when navigating to this page
        // let surveys = use_server_future(move || {
        //     let page = page();

        //     async move {
        //         match SurveyV2::get_client(config::get().api_url)
        //             .query(SurveyV2Query::new(size).with_page(page))
        //             .await
        //         {
        //             Ok(res) => res,
        //             Err(e) => {
        //                 tracing::error!("Failed to list surveys: {:?}", e);
        //                 QueryResponse::default()
        //             }
        //         }
        //     }
        // })?;

        let surveys = use_resource(move || {
            let page = page();

            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse::default();
                }

                match SurveyV2::get_client(config::get().api_url)
                    .query(
                        &org_id.unwrap().id,
                        SurveyV2Query::new(size).with_page(page),
                    )
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Failed to list surveys: {:?}", e);
                        QueryResponse::default()
                    }
                }
            }
        });

        let ctrl = Self {
            page,
            size,
            lang,
            surveys,
            popup_service: use_context(),
            translate: use_signal(|| translate),
        };

        Ok(ctrl)
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn page(&self) -> usize {
        (self.page)()
    }

    pub fn total_pages(&self) -> usize {
        self.surveys
            .with(|v| if let Some(v) = v { v.total_count } else { 0 }) as usize
    }

    pub fn get_surveys(&self) -> Option<QueryResponse<SurveyV2Summary>> {
        self.surveys.with(|v| v.clone())
    }

    pub async fn open_remove_survey_modal(&mut self, survey_id: String) {
        let mut popup_service = self.popup_service;
        let mut public_survey_resource = self.surveys;
        let translate = (self.translate)();

        // TODO: implement remove survey
        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang: self.lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onremove: {
                        move |_e: MouseEvent| {
                            let _survey_id = survey_id.clone();
                            public_survey_resource.restart();
                            popup_service.close();
                        }
                    },
                }
            })
            .with_id("remove_survey")
            .with_title(translate.remove_modal_title);
    }
}
