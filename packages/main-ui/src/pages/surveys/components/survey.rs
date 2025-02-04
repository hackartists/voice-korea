use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::{PublicSurveyQuestionType, Question};

use crate::{
    components::icons::{RowMenuDial, Trash},
    pages::surveys::{
        components::type_selection::SurveyTypeSelectionBox, i18n::SubjectiveTranslate,
    },
};

#[component]
pub fn QuestionListView(
    lang: Language,
    questions: Vec<Question>,
    types: Vec<String>,
    onchange_question: EventHandler<(usize, Question)>,
    onremove: EventHandler<usize>,
) -> Element {
    rsx! {
        for (index , survey) in questions.clone().iter().enumerate() {
            div {
                class: "flex flex-col w-full justify-start items-start pt-[5px] px-[40px] pb-[25px] bg-white rounded-[8px] mt-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row w-full justify-center items-center mb-[10px]",
                    RowMenuDial { width: "24", height: "24" }
                }

                Subjective {
                    lang,
                    selected_type: survey.question_type.translate(&lang),
                    types: types.clone(),
                    change_type: {
                        let survey = survey.clone();
                        move |survey_type: String| {
                            let survey_type = survey_type_from_str(survey_type.clone()).unwrap();
                            let mut survey = survey.clone();
                            survey.question_type = survey_type;
                            onchange_question.call((index, survey));
                        }
                    },

                    title: survey.title.clone(),
                    change_title: {
                        let survey = survey.clone();
                        move |title: String| {
                            let mut survey = survey.clone();
                            survey.title = title;
                            onchange_question.call((index, survey));
                        }
                    },

                    description: survey.description.clone().unwrap_or_default(),
                    change_description: {
                        let survey = survey.clone();
                        move |description: String| {
                            let mut survey = survey.clone();
                            survey.description = Some(description);
                            onchange_question.call((index, survey));
                        }
                    },

                    onremove: {
                        move |_| {
                            onremove.call(index);
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn Subjective(
    lang: Language,
    selected_type: String,
    types: Vec<String>,
    change_type: EventHandler<String>,

    title: String,
    change_title: EventHandler<String>,

    description: String,
    change_description: EventHandler<String>,

    onremove: EventHandler<MouseEvent>,
) -> Element {
    let translate: SubjectiveTranslate = translate(&lang);
    let mut is_focused = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-start items-center",
                SurveyTypeSelectionBox { selected_type, types, change_type }

                input {
                    class: format!(
                        "flex flex-row flex-1 h-[55px] justify-start items-center {} focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px]",
                        if (is_focused)() {
                            "bg-[#ffffff] border border-[#2a60d3]"
                        } else {
                            "bg-[#f7f7f7]"
                        },
                    ),
                    r#type: "text",
                    placeholder: translate.input_title_hint,
                    onfocus: move |_| {
                        is_focused.set(true);
                    },
                    onblur: move |_| {
                        is_focused.set(false);
                    },
                    value: title.clone(),
                    oninput: move |e: Event<FormData>| {
                        change_title.call(e.value());
                    },
                }
            }

            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            input {
                class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] mb-[20px]",
                r#type: "text",
                placeholder: translate.input_description_hint,
                value: description,
                oninput: move |e: Event<FormData>| {
                    change_description.call(e.value());
                },
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[5px]",
                button {
                    class: "flex flex-row w-[80px] items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-[#222222] text-[15px]", "{translate.remove}" }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}

pub fn survey_type_from_str(survey_type: String) -> Option<PublicSurveyQuestionType> {
    let survey_type = survey_type.parse::<PublicSurveyQuestionType>();

    match survey_type {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
