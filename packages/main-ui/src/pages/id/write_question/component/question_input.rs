#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::{
    components::{button::Button, select::Select},
    models::survey::StatusType,
    pages::id::write_question::controller::{Controller, ObjectiveQuestionOption, QuestionStep},
};

#[derive(Props, Clone, PartialEq)]
pub struct QuestionInputProps {
    lang: Language,
    temporary_save: String,
    input_question: String,
    next_question: String,
    save_label: String,
    cancel_label: String,
    enter_subject_enter_text: String,
    add_option: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ObjectiveQuestionProps {
    objective_questions: Vec<ObjectiveQuestionOption>,
    add_option: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct SubjectiveQuestionProps {
    enter_subject_enter_text: String,
}

pub fn QuestionInput(props: QuestionInputProps) -> Element {
    let mut ctrl: Controller = use_context();
    let survey = ctrl.get_survey();
    let questions = ctrl.get_question_types();
    let selected_question = ctrl.get_selected_question();
    tracing::debug!(
        "selected question: {:?} {}",
        selected_question,
        questions
            .get(selected_question as usize)
            .unwrap()
            .clone()
            .value
    );
    let objective_questions = ctrl.get_objective_questions();
    rsx! {
        Fragment {
            div { class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                div { class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                    "{survey.title}"
                }
            }
            div { class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                div { class: "flex flex-row w-full",
                    input {
                        class: "flex flex-1 text-[21px] text-[#8a8a8a] font-normal",
                        "type": "text",
                        style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none; border-bottom: 1px solid #9f9f9f;",
                        placeholder: props.input_question.clone(),
                        value: ctrl.get_question_title().clone(),
                        onchange: move |e| {
                            ctrl.change_question_title(e.value());
                        },
                    }
                    Select {
                        class: "w-[205px] h-[50px] rounded-lg font-normal text-[16px] text-black pt-[10px] pb-[10px] pl-[13px] pr-[13px] ml-[10px]",
                        border: "1px solid #9f9f9f",
                        value: selected_question as i64,
                        onchange: move |evt: Event<FormData>| {
                            let data = evt.clone().value();
                            ctrl.change_selected_question(data.parse::<u64>().unwrap_or(0));
                        },
                        component: rsx! {
                            {
                                questions
                                    .iter()
                                    .map(|opt| {
                                        rsx! {
                                            option {
                                                label: opt.label.clone(),
                                                value: opt.value.clone(),
                                                color: "#000000",
                                                selected: if opt.value.clone() == selected_question as i64 { true } else { false },
                                            }
                                        }
                                    })
                            }
                        },
                    }
                }
                if selected_question == 0 {
                    ObjectiveQuestion { objective_questions, add_option: props.add_option }
                } else {
                    SubjectiveQuestion { enter_subject_enter_text: props.enter_subject_enter_text }
                }
                div { class: "flex flex-row w-full justify-between items-center mt-[30px]",
                    Button {
                        button_text: props.next_question,
                        onclick: move |_| async move {
                            ctrl.write_question(StatusType::TemporarySave).await;
                            ctrl.refresh_survey_response();
                            ctrl.clear_data();
                        },
                        class: "flex flex-row w-[120px] h-[50px] bg-[#3a94ff]",
                    }
                    div { class: "flex flex-row justify-center items-center",
                        Button {
                            button_text: props.cancel_label,
                            onclick: move |_| {
                                ctrl.clear_data();
                                ctrl.change_step(QuestionStep::List);
                            },
                            class: "flex flex-row w-[80px] h-[50px] bg-[#434343] mr-[10px]",
                        }
                        Button {
                            button_text: props.save_label,
                            onclick: move |_| async move {
                                ctrl.write_question(StatusType::TemporarySave).await;
                                ctrl.refresh_survey_response();
                                ctrl.clear_data();
                                ctrl.change_step(QuestionStep::List);
                            },
                            class: "flex flex-row w-[80px] h-[50px] bg-[#2168c3]",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SubjectiveQuestion(props: SubjectiveQuestionProps) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-center items-start mt-[30px]",
            div { class: "flex flex-row w-full justify-start items-center mb-[20px]",
                input {
                    r#type: "radio",
                    style: "height:18px; width:18px; vertical-align: middle",
                    name: "subjective",
                }
                div {
                    class: "flex flex-1 max-w-[890px] min-w-[300px] text-[21px] text-[#8a8a8a] font-normal; mr-[15px]; ml-[15px]",
                    style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none; border-bottom: 1px solid #9f9f9f;",
                    {props.enter_subject_enter_text}
                }
            }
        }
    }
}

#[component]
pub fn ObjectiveQuestion(props: ObjectiveQuestionProps) -> Element {
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col w-full justify-center items-start mt-[30px]",
            div { class: "flex flex-col w-full",
                for i in 0..props.objective_questions.len() {
                    div {
                        key: format!("list-{:?}", i),
                        class: "flex flex-row w-full justify-start items-center mb-[20px]",
                        input {
                            r#type: "radio",
                            style: "height:18px; width:18px; vertical-align: middle",
                            name: "objective",
                        }
                        input {
                            class: "flex flex-1 max-w-[890px] min-w-[300px] text-[21px] text-[#8a8a8a] font-normal; mr-[15px]",
                            "type": "text",
                            style: "margin-left: 15px; border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none; border-bottom: 1px solid #9f9f9f;",
                            placeholder: props.objective_questions.get(i).unwrap().clone().hint,
                            value: props.objective_questions.get(i).unwrap().clone().text_value,
                            onchange: move |e| {
                                ctrl.change_objective_question(i, e.value());
                            },
                        }
                        img {
                            src: asset!("/public/images/remove.png"),
                            onclick: move |_| {
                                ctrl.remove_objective_question(i);
                            },
                            class: "w-[25px] h-[25px]",
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[20px] mt-[10px]",
                input {
                    r#type: "radio",
                    style: "height:18px; width:18px; vertical-align: middle",
                    name: "objective",
                }
                div {
                    class: "flex flex-1 max-w-[890px] min-w-[300px] text-[20px] text-[#3a94ff] font-normal; mr-[15px]; ml-[15px]",
                    onclick: move |_| {
                        tracing::info!("this button clicked");
                        ctrl.add_objective_question();
                    },
                    div { {props.add_option} }
                }
            }
        }
    }
}
