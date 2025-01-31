use dioxus::prelude::ServerFnError;
use dioxus::prelude::*;
use models::prelude::PublicSurveySummary;

use crate::api::common::CommonQueryResponse;
use crate::service::survey_api::SurveyApi;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    public_survey_resource:
        Resource<Result<CommonQueryResponse<PublicSurveySummary>, ServerFnError>>,
    surveys: Signal<Vec<PublicSurveySummary>>,
}

impl Controller {
    pub fn new(_lang: dioxus_translate::Language) -> Self {
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
}
