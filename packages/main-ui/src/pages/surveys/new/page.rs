use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::{
        models::current_step::CurrentStep,
        new::{
            controller::Controller, create_survey::CreateSurvey, i18n::SurveyNewTranslate,
            setting_panel::SettingPanel,
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

                CreateSurvey {
                    lang: props.lang,
                    visibility: ctrl.get_current_step() == CurrentStep::CreateSurvey,
                    onnext: move |req| ctrl.handle_survey_request(req),
                }
                SettingPanel {
                    lang: props.lang,
                    visibility: ctrl.get_current_step() == CurrentStep::SettingPanel,
                }
            }
        }
    }
}
