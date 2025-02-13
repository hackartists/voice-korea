use dioxus::prelude::*;
use dioxus_translate::*;

use crate::pages::surveys::new::page::SurveyCreatePage;

#[component]
pub fn SurveyUpdatePage(lang: Language, survey_id: i64) -> Element {
    rsx! {
        SurveyCreatePage { lang, survey_id }
    }
}
