use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{
    components::icons::ArrowLeft, pages::surveys::components::introduction::InputIntroduction,
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyCreateProps {
    lang: Language,
}

#[component]
pub fn SurveyCreatePage(props: SurveyCreateProps) -> Element {
    let _props = props.clone();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]", "조사 관리" }
            div { class: "flex flex-row w-full justify-start items-center mb-[40px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::SurveyPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#555462" }
                }
                div { class: "text-[#222222] font-semibold text-[28px]", "조사 시작하기" }
            }
            InputIntroduction {}
        }
    }
}
