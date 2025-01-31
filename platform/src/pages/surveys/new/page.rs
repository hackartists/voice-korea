use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{ArrowLeft, Plus},
    pages::surveys::{
        components::introduction::InputIntroduction,
        new::{
            controller::Controller,
            i18n::{AddQuestionTranslate, SurveyNewTranslate},
        },
    },
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyCreateProps {
    lang: Language,
}

#[component]
pub fn SurveyCreatePage(props: SurveyCreateProps) -> Element {
    let translates: SurveyNewTranslate = translate(&props.lang);
    let mut ctrl = Controller::new(props.lang);
    rsx! {
        div { class: "relative h-full grow",
            div { class: "flex flex-col w-full h-full justify-start items-start",
                div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]",
                    "{translates.survey_title}"
                }
                div { class: "flex flex-row w-full justify-start items-center mb-[40px]",
                    Link {
                        class: "mr-[6px]",
                        to: Route::SurveyPage {
                            lang: props.lang,
                        },
                        ArrowLeft { width: "24", height: "24", color: "#555462" }
                    }
                    div { class: "text-[#222222] font-semibold text-[28px]",
                        "{translates.start_survey}"
                    }
                }
                InputIntroduction {
                    lang: props.lang,
                    selected_field: ctrl.get_selected_field(),
                    fields: ctrl.get_total_fields(),
                    change_field: move |field: String| {
                        ctrl.change_selected_field(field);
                    },

                    title: ctrl.get_title(),
                    change_title: move |title: String| {
                        ctrl.change_title(title);
                    },

                    start_date: ctrl.get_start_date(),
                    change_start_date: move |start_date: i64| {
                        ctrl.change_start_date(start_date);
                    },

                    end_date: ctrl.get_end_date(),
                    change_end_date: move |end_date: i64| {
                        ctrl.change_end_date(end_date);
                    },

                    description: ctrl.get_description(),
                    change_description: move |description: String| {
                        ctrl.change_description(description);
                    },
                }

                AddQuestion { lang: props.lang }
            }

            div { class: "absolute right-[0px] bottom-[40px]",
                div {
                    class: "flex flex-row w-[70px] h-[70px] justify-center items-center bg-white border border-[#ebeff5] rounded-[100px]",
                    style: "box-shadow: 0px 8px 15px 0px rgba(53, 70, 177, 0.45);",
                    Plus { width: "18", height: "18", color: "#555462" }
                }
            }
        }
    }
}

#[component]
pub fn AddQuestion(lang: Language) -> Element {
    let translates: AddQuestionTranslate = translate(&lang);
    rsx! {
        button { class: "flex flex-col w-full h-[200px] rounded-[8px] justify-center items-center border border-dashed border-[#b4b4b4] mt-[20px]",
            div { class: "flex flex-row w-[45px] h-[45px] justify-center items-center rounded-[100px] border border-[#b4b4b4]",
                Plus { width: "12", height: "12", color: "#b4b4b4" }
            }
            div { class: "mt-[10px] font-medium text-[15px] text-[#b4b4b4] leading-[22px]",
                "{translates.add_description}"
            }
        }
    }
}
