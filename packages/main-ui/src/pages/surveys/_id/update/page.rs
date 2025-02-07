use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::{
        _id::update::{
            controller::Controller, i18n::SurveyUpdateTranslate, update_panels::UpdatePanels,
            update_survey::UpdateSurvey,
        },
        models::current_step::CurrentStep,
    },
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyUpdateProps {
    lang: Language,
    survey_id: String,
}

#[component]
pub fn SurveyUpdatePage(props: SurveyUpdateProps) -> Element {
    let tr: SurveyUpdateTranslate = translate(&props.lang);
    let ctrl = Controller::new(props.lang, props.survey_id.parse::<i64>().unwrap());

    let step = ctrl.get_current_step();

    rsx! {
        div { class: "flex flex-col gap-[40px] items-end justify-start mb-[40px]",
            div { class: "flex flex-col w-full h-full justify-start items-start",
                div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]", "{tr.survey_title}" }
                div { class: "flex flex-row w-full justify-start items-center mb-[40px]",
                    Link {
                        class: "mr-[6px]",
                        to: Route::SurveyPage {
                            lang: props.lang,
                        },
                        ArrowLeft { width: "24", height: "24", color: "#555462" }
                    }
                    div { class: "text-[#222222] font-semibold text-[28px]", "test" }
                }

                if step == CurrentStep::CreateSurvey {
                    UpdateSurvey { lang: props.lang }
                } else {
                    UpdatePanels { lang: props.lang }
                }
            }
        }
    }
}
