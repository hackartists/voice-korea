use dioxus::prelude::*;
use dioxus_translate::Language;
use models::response::SurveyResponse;

#[derive(Clone, Copy)]
pub struct Controller {
    survey_id: Signal<i64>,
    survey: Signal<SurveyResponse>,
}

impl Controller {
    pub fn new(_lang: Language, survey_id: i64) -> Self {
        let survey = SurveyResponse {
            id: 0,
            created_at: 0,
            updated_at: 0,
            proof_id: "".to_string(),
            title: "조사 제목명".to_string(),
            start_date: 1739286000,
            end_date: 1741705200,
            total_response_count: 1720,
            response_count: 1454,
            average_time: "00:02:00".to_string(),
            attributes: vec![],
            answers: vec![],
            survey_id,
        };
        let ctrl = Self {
            survey_id: use_signal(|| survey_id),
            survey: use_signal(|| survey),
        };

        ctrl
    }

    pub fn get_survey(&self) -> SurveyResponse {
        (self.survey)()
    }

    pub fn get_survey_id(&self) -> i64 {
        (self.survey_id)()
    }
}
