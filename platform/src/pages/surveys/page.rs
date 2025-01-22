use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Props, Clone, PartialEq)]
pub struct SurveyProps {
    lang: Language,
}

#[component]
pub fn SurveyPage(props: SurveyProps) -> Element {
    let _props = props.clone();
    rsx! {
        div { "survey page" }
    }
}
