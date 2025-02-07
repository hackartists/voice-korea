use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{prelude::Question, ProjectArea};

use crate::{
    components::icons::Plus,
    pages::surveys::{
        _id::update::{controller::Controller, i18n::UpdateSurveyTranslate},
        components::{introduction::InputIntroduction, survey::QuestionListView},
        models::current_step::CurrentStep,
        new::i18n::AddQuestionTranslate,
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct UpdateSurveyProps {
    lang: Language,
}

#[component]
pub fn UpdateSurvey(props: UpdateSurveyProps) -> Element {
    let translates: UpdateSurveyTranslate = translate(&props.lang);
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-start",
            div { class: "flex flex-col w-full",
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

                    selected: ctrl.get_selected_field(),
                    started: ctrl.get_start_date(),
                    ended: ctrl.get_end_date(),
                    ti: ctrl.get_title(),
                    desc: ctrl.get_description(),
                }

                QuestionListView {
                    lang: props.lang,
                    questions: ctrl.get_surveys(),
                    onchange: move |(index, survey): (usize, Question)| {
                        ctrl.change_survey(index, survey);
                    },
                    onremove: move |index: usize| {
                        ctrl.remove_survey(index);
                    },
                }

                button {
                    class: "flex flex-row w-full",
                    onclick: move |_| {
                        ctrl.add_question();
                    },
                    AddQuestion { lang: props.lang }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[20px] text-white mt-[40px]",
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| {
                        ctrl.back();
                    },
                    "{translates.btn_cancel}"
                }
                // button {
                //     class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                //     onclick: move |_| async move {
                //         ctrl.save_survey().await;
                //     },
                //     "{translates.btn_temp_save}"
                // }

                button {
                    class: "px-[20px] py-[10px] bg-[#2A60D3] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.change_step(CurrentStep::SettingPanel);
                    },
                    "{translates.btn_next}"
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
