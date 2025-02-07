use dioxus::prelude::*;

use dioxus_translate::{translate, Language};
use models::prelude::Question;

use crate::{
    components::icons::{Minus, RowMenuDial, Trash},
    pages::surveys::{
        components::type_selection::QuestionTypeSelector,
        i18n::{ObjectiveTranslate, QuestionListViewTranslate, SubjectiveTranslate},
    },
};

#[component]
pub fn QuestionListView(
    lang: Language,
    questions: Vec<Question>,
    onchange: EventHandler<(usize, Question)>,
    onremove: EventHandler<usize>,
) -> Element {
    let tr: QuestionListViewTranslate = translate(&lang);
    rsx! {
        for index in 0..questions.len() {
            div {
                class: "flex flex-col w-full justify-start items-start pt-[5px] px-[40px] pb-[25px] bg-white rounded-[8px] mt-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row w-full justify-center items-center mb-[10px]",
                    RowMenuDial { width: "24", height: "24" }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-start items-center",
                        QuestionTypeSelector {
                            lang,
                            onchange: {
                                move |qtype: String| {
                                    let question = Question::new(&qtype);
                                    onchange((index, question.clone()));
                                }
                            },
                        }

                        input {
                            class: format!(
                                "flex flex-row flex-1 h-[55px] justify-start items-center bg-[#f7f7f7] focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px]",
                            ),
                            r#type: "text",
                            placeholder: "{tr.input_title}",
                            value: questions[index].title(),
                            oninput: {
                                let mut question = questions[index].clone();
                                move |e: Event<FormData>| {
                                    question.set_title(&e.value());
                                    onchange((index, question.clone()));
                                }
                            },
                        }
                    }

                    if matches!(questions[index], Question::ShortAnswer(_) | Question::Subjective(_)) {
                        Subjective {
                            lang,
                            onchange: move |q: Question| {
                                onchange.call((index, q));
                            },
                            onremove: move |_| {
                                onremove.call(index);
                            },
                            question: questions[index].clone(),
                        }
                    } else {
                        Objective {
                            lang,
                            onchange: move |q: Question| {
                                onchange.call((index, q));
                            },
                            onremove: move |_| {
                                onremove.call(index);
                            },
                            question: questions[index].clone(),
                        }
                    }
                }

            }
        }
    }
}

#[component]
pub fn Objective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let options = question.options();
    let tr: ObjectiveTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            for (index , option) in options.iter().enumerate() {
                div { class: "flex flex-row w-full justify-start items-center mt-[10px]",
                    div { class: "flex flex-row w-[20px] h-[20px] border-[3px] bg-white border-[#9f9f9f] mr-[10px] rounded-[100px]" }
                    input {
                        class: "flex flex-row w-[888px] h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#9f9f9f] px-[15px] py-[15px] font-medium text-[#9f9f9f] text-[15px] leading-[22px] mr-[10px]",
                        r#type: "text",
                        placeholder: format!("{} {}", tr.option, index),
                        value: option.clone(),
                        oninput: {
                            let mut question = question.clone();
                            move |e: Event<FormData>| {
                                question.change_option(index, &e.value());
                                onchange.call(question.clone());
                            }
                        },
                    }
                    button {
                        onclick: {
                            let mut question = question.clone();
                            move |_| {
                                question.remove_option(index);
                                onchange.call(question.clone());
                            }
                        },
                        Minus { width: "20", height: "20" }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-start items-center mt-[30px]",
                div { class: "flex flex-row w-[20px] h-[20px] border-[3px] bg-white border-[#9f9f9f] mr-[10px] rounded-[100px]" }
                button {
                    class: "font-medium text-[16px] text-[#3a94ff]",
                    onclick: {
                        let mut question = question.clone();
                        move |_| {
                            question.add_option("");
                            onchange.call(question.clone());
                        }
                    },
                    "{tr.add_option}"
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[5px] mt-[10px]",
                button {
                    class: "flex flex-row w-[80px] items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-[#222222] text-[15px]", "{tr.remove}" }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}

#[component]
pub fn Subjective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let tr: SubjectiveTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            input {
                class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] mb-[20px]",
                r#type: "text",
                placeholder: tr.input_description_hint,
                value: question.description(),
                oninput: move |e: Event<FormData>| {
                    question.set_description(&e.value());
                    onchange.call(question.clone());
                },
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[5px]",
                button {
                    class: "flex flex-row w-[80px] items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-[#222222] text-[15px]", "{tr.remove}" }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}
