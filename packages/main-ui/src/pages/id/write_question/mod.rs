#![allow(non_snake_case)]
use component::{question_input::QuestionInput, question_list::QuestionList};
use controller::QuestionStep;
use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::WriteQuestionTranslate;

mod controller;
mod i18n;
pub mod component {
    pub mod question_input;
    pub mod question_list;
}

#[derive(PartialEq, Props, Clone)]
pub struct WriteQuestionProps {
    lang: Language,
    survey_id: String,
}

#[component]
pub fn WriteQuestionPage(props: WriteQuestionProps) -> Element {
    let mut ctrl = controller::Controller::init(props.lang.clone(), props.survey_id.clone());
    let translates: WriteQuestionTranslate = translate(&props.lang.clone());
    let step = ctrl.get_step();

    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-center",
            if step == QuestionStep::List {
                div { class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                    QuestionList {
                        survey_id: props.survey_id,
                        lang: props.lang,
                        delete: translates.delete,
                        update: translates.update,
                        temporary_save: translates.temporary_save,
                        add_question: translates.add_question,
                        back: translates.back_label,
                        save: translates.save_label,
                    }
                }
            } else {
                div { class: "flex flex-col max-w-7xl min-w-md w-full justify-start items-start mt-4 px-12",
                    QuestionInput {
                        lang: props.lang,
                        temporary_save: translates.temporary_save,
                        input_question: translates.input_question,
                        next_question: translates.next_question,
                        save_label: translates.save_label,
                        cancel_label: translates.cancel_label,
                        enter_subject_enter_text: translates.enter_subject_enter_text,
                        add_option: translates.add_option,
                    }
                }
            }
        }
    }
}
