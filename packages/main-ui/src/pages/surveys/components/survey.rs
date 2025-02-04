use dioxus::prelude::*;

use dioxus_translate::{translate, Language};
use models::prelude::Question;

use crate::{
    components::icons::{RowMenuDial, Trash},
    pages::surveys::{components::type_selection::QuestionTypeSelector, i18n::SubjectiveTranslate},
};

#[component]
pub fn QuestionListView(
    lang: Language,
    questions: Vec<Question>,
    onchange: EventHandler<(usize, Question)>,
    onremove: EventHandler<usize>,
) -> Element {
    rsx! {
        for index in 0..questions.len() {
            div {
                class: "flex flex-col w-full justify-start items-start pt-[5px] px-[40px] pb-[25px] bg-white rounded-[8px] mt-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row w-full justify-center items-center mb-[10px]",
                    RowMenuDial { width: "24", height: "24" }
                }

                Subjective {
                    lang,
                    onchange: move |q: Question| {
                        onchange.call((index, q));
                    },
                    onremove: move |_| {
                        onremove.call(index);
                    },
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
) -> Element {
    let translate: SubjectiveTranslate = translate(&lang);
    let mut is_focused = use_signal(|| false);
    let mut question = use_signal(|| Question::default());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-start items-center",
                QuestionTypeSelector {
                    lang,
                    onchange: move |qtype: String| {
                        question.set(Question::new(&qtype));
                        onchange(question());
                    },
                }

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
                    value: question().title(),
                    onfocus: move |_| {
                        is_focused.set(true);
                    },
                    onblur: move |_| {
                        is_focused.set(false);
                    },
                    oninput: move |e: Event<FormData>| {
                        question.write().set_title(&e.value());
                        onchange.call(question());
                    },
                }
            }

            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            input {
                class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] mb-[20px]",
                r#type: "text",
                placeholder: translate.input_description_hint,
                value: question().description(),
                oninput: move |e: Event<FormData>| {
                    question.write().set_description(&e.value());
                    onchange.call(question());
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
