use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::_id::{controller::Controller, i18n::SurveyResultTranslate},
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyResultProps {
    lang: Language,
    survey_id: i64,
}

#[component]
pub fn SurveyResultPage(props: SurveyResultProps) -> Element {
    let tr: SurveyResultTranslate = translate(&props.lang);
    let _ctrl = Controller::new(props.lang, props.survey_id);

    rsx! {
        div { class: "flex flex-col gap-[40px] items-end justify-start mb-[40px]",
            div { class: "flex flex-col w-full h-full justify-start items-start",
                div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]",
                    "{tr.survey_management} / {tr.update_survey}"
                }
                div { class: "flex flex-row w-full justify-start items-center mb-[40px]",
                    Link {
                        class: "mr-[6px]",
                        to: Route::SurveyPage {
                            lang: props.lang,
                        },
                        ArrowLeft { width: "24", height: "24", color: "#555462" }
                    }
                    div { class: "text-[#222222] font-semibold text-[28px]", "조사 제목명" }
                }

                div { class: "flex flex-row w-full justify-start items-start gap-[10px]",
                    SurveyResponseBox { title: "{tr.total_survey_target}", value: "1,720" }
                    SurveyResponseBox { title: "{tr.number_of_responses}", value: "1,454" }
                    SurveyResponseBox { title: "{tr.rate_of_responses}", value: "98%" }
                    SurveyResponseBox { title: "{tr.average_time_taken}", value: "00:02:00" }
                    SurveyResponseBox { title: "{tr.remaining_period}", value: "20일" }
                    SurveyResponseBox {
                        title: "{tr.survey_period}",
                        value: "02.12 - 03.12 / 2025",
                    }
                }
            }
        }
    }
}

#[component]
pub fn SurveyResponseBox(title: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col justify-center items-center py-[18px] px-[24px] gap-[15px] rounded-[8px] border border-[#ebeff5] bg-[#ffffff]",
            div { class: "font-semibold text-[#35343f] text-[15px] leading-[18px] mb-[15px]",
                "{title}"
            }
            div { class: "font-bold text-[#435393] textr-[24px] leading-[30px]", "{value}" }
        }
    }
}
