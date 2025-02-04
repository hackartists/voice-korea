use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{prelude::PublicSurveyQuestion, ProjectArea};

use crate::{
    components::icons::{ArrowLeft, Plus},
    pages::surveys::{
        components::{introduction::InputIntroduction, survey::ListSurvey},
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
        div { class: "flex flex-col gap-[40px] items-end justify-start mb-[40px]",
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
                    onchange_area: move |field: ProjectArea| {
                        ctrl.change_selected_field(field);
                    },

                    onchange_title: move |title: String| {
                        ctrl.change_title(title);
                    },

                    onchange_start_date: move |start_date: i64| {
                        ctrl.change_start_date(start_date);
                    },

                    onchange_end_date: move |end_date: i64| {
                        ctrl.change_end_date(end_date);
                    },

                    onchange_description: move |description: String| {
                        ctrl.change_description(description);
                    },
                }

                ListSurvey {
                    lang: props.lang,
                    surveys: ctrl.get_surveys(),
                    types: ctrl.get_total_survey_types(),
                    change_survey: move |(index, survey): (usize, PublicSurveyQuestion)| {
                        ctrl.change_survey(index, survey);
                    },
                    onremove: move |index: usize| {
                        ctrl.remove_survey(index);
                    },
                }

                button {
                    class: "flex flex-row w-full",
                    onclick: move |_| {
                        ctrl.add_survey();
                    },
                    AddQuestion { lang: props.lang }
                }
            }

            div { class: "flex flex-row gap-[20px] text-white",
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| {
                        ctrl.back();
                    },
                    "{translates.btn_cancel}"
                }
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.save_survey().await;
                    },
                    "{translates.btn_temp_save}"
                }

                button {
                    class: "px-[20px] py-[10px] bg-[#2A60D3] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.save_survey().await;
                    },
                    "{translates.btn_complete}"
                }
            }
        }
    }
}

#[component]
pub fn AddQuestion(lang: Language) -> Element {
    let translates: AddQuestionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full h-[200px] rounded-[8px] justify-center items-center border border-dashed border-[#b4b4b4] mt-[20px]",
            div { class: "flex flex-row w-[45px] h-[45px] justify-center items-center rounded-[100px] border border-[#b4b4b4]",
                Plus { width: "12", height: "12", color: "#b4b4b4" }
            }
            div { class: "mt-[10px] font-medium text-[15px] text-[#b4b4b4] leading-[22px]",
                "{translates.add_description}"
            }
        }
    }
}
