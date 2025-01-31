use chrono::{TimeZone, Utc};
use dioxus::prelude::ServerFnError;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::{Field, PublicSurveyStatus, PublicSurveySummary, SurveyType};

use crate::api::common::CommonQueryResponse;
use crate::pages::surveys::page::RemoveSurveyModal;
use crate::service::popup_service::PopupService;
use crate::service::survey_api::SurveyApi;

use super::i18n::SurveyTranslate;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    public_survey_resource:
        Resource<Result<CommonQueryResponse<PublicSurveySummary>, ServerFnError>>,
    surveys: Signal<Vec<PublicSurveySummary>>,
    popup_service: Signal<PopupService>,
    public_survey_api: SurveyApi,
    translate: Signal<SurveyTranslate>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let translate: SurveyTranslate = translate(&lang);
        let public_survey_api: SurveyApi = use_context();

        let public_survey_resource: Resource<
            Result<CommonQueryResponse<models::prelude::PublicSurveySummary>, ServerFnError>,
        > = use_resource(move || {
            let api = public_survey_api.clone();

            //FIXME: add bookmark
            async move {
                let res = api.list_surveys(Some(100), None).await;

                res
            }
        });

        let mut ctrl = Self {
            public_survey_resource,
            surveys: use_signal(|| vec![]),
            popup_service: use_signal(|| popup_service),
            public_survey_api,
            translate: use_signal(|| translate),
        };

        match public_survey_resource.value()() {
            Some(surveys) => {
                if surveys.is_ok() {
                    ctrl.surveys.set(surveys.unwrap().items);
                }
            }
            _ => {}
        }
        ctrl
    }

    pub fn get_surveys(&self) -> Vec<PublicSurveySummary> {
        (self.surveys)()
    }

    pub fn translate_survey_type(&self, lang: Language, survey_type: SurveyType) -> &'static str {
        survey_type.translate(&lang)
    }

    pub fn translate_survey_field(&self, lang: Language, survey_field: Field) -> &'static str {
        survey_field.translate(&lang)
    }

    pub fn translate_survey_status(
        &self,
        lang: Language,
        survey_status: PublicSurveyStatus,
    ) -> &'static str {
        survey_status.translate(&lang)
    }

    pub fn convert_timestamp_to_date(&self, timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub async fn open_remove_survey_modal(&self, lang: Language, survey_id: String) {
        let mut popup_service = (self.popup_service)().clone();
        let api: SurveyApi = self.public_survey_api;

        let mut public_survey_resource = self.public_survey_resource;
        let translate = (self.translate)();

        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onremove: {
                        move |_e: MouseEvent| {
                            let survey_id = survey_id.clone();
                            async move {
                                let _ = api.remove_survey(survey_id).await;
                                public_survey_resource.restart();
                                popup_service.close();
                            }
                        }
                    },
                }
            })
            .with_id("remove_survey")
            .with_title(translate.remove_modal_title);
    }
}
