use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Props, Clone, PartialEq)]
pub struct SurveyCreateProps {
    lang: Language,
}

#[component]
pub fn SurveyCreatePage(props: SurveyCreateProps) -> Element {
    let _props = props.clone();
    rsx! {
        div { "survey create page" }
    }
}
