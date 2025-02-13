use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    excel::SurveyResponseExcel,
    response::{Answer, SurveyResponse, SurveyResponseQuery, SurveyResponseSummary},
    QueryResponse, SurveyV2,
};

use crate::service::login_service::LoginService;

#[derive(Clone, Copy)]
pub struct Controller {
    survey_id: i64,
    org_id: Memo<i64>,
    surveys: Resource<SurveyV2>,
    responses: Resource<QueryResponse<SurveyResponseSummary>>,
    endpoint: &'static str,
}

impl Controller {
    pub fn new(_lang: Language, survey_id: i64) -> Self {
        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);

        let surveys: Resource<SurveyV2> = use_resource(move || {
            let org_id = org_id();

            async move {
                let cli = SurveyV2::get_client(&crate::config::get().api_url);

                match cli.get(org_id, survey_id).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                        SurveyV2::default()
                    }
                }
            }
        });

        let responses: Resource<QueryResponse<SurveyResponseSummary>> = use_resource(move || {
            async move {
                let cli = SurveyResponse::get_client(&crate::config::get().api_url);

                // FIXME: this is workaround only for testing
                //        fix to apply page
                match cli.query(survey_id, SurveyResponseQuery::new(10000)).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                        QueryResponse::default()
                    }
                }
            }
        });

        let endpoint = crate::config::get().api_url;

        let ctrl = Self {
            survey_id,
            org_id,
            surveys,
            endpoint,
            responses,
        };

        ctrl
    }

    pub fn responses(&self) -> Option<QueryResponse<SurveyResponseSummary>> {
        self.responses.value()()
    }

    pub fn get_survey(&self) -> Option<SurveyV2> {
        self.surveys.value()()
    }

    pub async fn simulate_response(&self) {
        let cli = SurveyResponse::get_client(self.endpoint);
        let survey = self.get_survey().unwrap();
        for i in 0..survey.panels.len() {
            let attrs = models::response::Attribute::from_panel(&survey.panels[i]);
            let quota = survey.panel_counts[i].user_count;

            for j in 0..quota {
                let res = cli
                    .respond_answer(
                        self.survey_id,
                        "proof_id".to_string(),
                        attrs.clone(),
                        survey.questions.iter().map(Answer::simulate).collect(),
                    )
                    .await;
                if res.is_err() {
                    tracing::error!("you might already make some answers error: {:?}", res);
                } else {
                    tracing::info!(
                        "{}-th Response created for panel({})",
                        j,
                        survey.panels[i].id
                    );
                }
            }
        }
    }

    pub async fn download_excel(&self) {
        let cli = SurveyResponseExcel::get_client(self.endpoint);

        let res = cli.download_excel((self.org_id)(), self.survey_id).await;

        if let Ok(res) = res {
            tracing::debug!("download link {:?}", res.url);
            #[cfg(feature = "web")]
            {
                use wasm_bindgen::JsCast;

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &res.url).unwrap();
                a.set_attribute("download", &format!("survey-{}.xlsx", self.survey_id))
                    .unwrap();

                document.body().unwrap().append_child(&a).unwrap();
                let a: web_sys::HtmlElement = a.unchecked_into();
                a.click();
                a.remove();
            }
        } else {
            tracing::error!("Error: {:?}", res);
        }
    }
}
