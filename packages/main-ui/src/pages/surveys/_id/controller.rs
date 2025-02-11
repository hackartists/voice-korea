use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Copy)]
pub struct Controller {
    survey_id: Signal<i64>,
}

impl Controller {
    pub fn new(_lang: Language, survey_id: i64) -> Self {
        let ctrl = Self {
            survey_id: use_signal(|| survey_id),
        };

        ctrl
    }

    pub fn get_survey_id(&self) -> i64 {
        (self.survey_id)()
    }
}
