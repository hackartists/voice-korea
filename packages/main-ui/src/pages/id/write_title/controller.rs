#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::{Survey, SurveyDraftStatus, UpsertSurveyDraftRequest};

use crate::{models::survey::StatusType, service::prev_survey_api::PrevSurveyApi};

use super::Language;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    survey: Signal<Survey>,
    pub survey_title: Signal<String>,
    pub survey_id: Signal<String>,

    pub prev_survey_api: PrevSurveyApi,
}

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        #[cfg(feature = "web")]
        {
            use crate::routes::Route;
            use crate::service::login_service::use_login_service;

            let navigator = use_navigator();

            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let prev_survey_api: PrevSurveyApi = use_context();

        let mut ctrl = Self {
            survey: use_signal(|| Survey::default()),
            survey_title: use_signal(|| "".to_string()),
            survey_id: use_signal(|| "".to_string()),
            prev_survey_api,
        };

        ctrl.survey_id.set(id.clone());

        let _ = use_effect(move || {
            let id_value = id.clone();
            let prev_survey_api = prev_survey_api.clone();
            spawn(async move {
                let _ = async move {
                    match prev_survey_api.get_survey(id_value).await {
                        Ok(res) => {
                            tracing::debug!("survey title: {}", res.clone().title.clone());

                            ctrl.survey_title.set(res.clone().title.clone());
                            ctrl.survey.set(res);
                        }
                        Err(e) => {
                            tracing::error!("Error: {:?}", e);
                        }
                    }
                }
                .await;
            });
        });

        let draft_status = ctrl.get_survey().draft_status;
        let title = ctrl.get_survey().title;

        // if (!draft_status.is_none() && draft_status != Some(SurveyDraftStatus::Title))
        //     || (draft_status.is_none() && title != "")
        // {
        //     navigator.push(Route::DashboardPage { lang });
        // };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey_title(&self) -> String {
        (self.survey_title)()
    }

    pub fn set_survey_title(&mut self, title: String) {
        self.survey_title.set(title);
    }

    pub fn get_survey_id(&self) -> String {
        (self.survey_id)()
    }

    pub fn get_survey(&self) -> Survey {
        (self.survey)()
    }

    pub async fn write_survey_title(&mut self, status: StatusType, title: String) {
        tracing::info!("write survey title button clicked {title}");

        if status == StatusType::TemporarySave {
            let _ = self
                .prev_survey_api
                .upsert_survey_draft(UpsertSurveyDraftRequest {
                    id: Some(self.get_survey_id()),
                    status: Some(SurveyDraftStatus::Title),
                    title: Some(title.clone()),
                    quotas: None,
                    questions: None,
                    started_at: None,
                    ended_at: None,
                })
                .await;
        } else {
            let _ = self
                .prev_survey_api
                .upsert_survey_draft(UpsertSurveyDraftRequest {
                    id: Some(self.get_survey_id()),
                    status: Some(SurveyDraftStatus::Question),
                    title: Some(title.clone()),
                    quotas: None,
                    questions: None,
                    started_at: None,
                    ended_at: None,
                })
                .await;
        }
    }
}
