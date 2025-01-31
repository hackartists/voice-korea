#![allow(non_snake_case)]
use components::{response_list::ResponseList, response_summary::ResponseSummary};
use controller::SelectPage;
use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::ResponseReportTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct ResponseReportProps {
    lang: Language,
    survey_id: String,
}

pub mod controller;
pub mod i18n;
pub mod components {
    pub mod response_list;
    pub mod response_summary;
}

#[component]
pub fn ResponseReportPage(props: ResponseReportProps) -> Element {
    let mut ctrl = controller::Controller::init(props.lang, props.survey_id);
    let translates: ResponseReportTranslate = translate(&props.lang.clone());
    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-center",
            div { class: "flex flex-col max-w-[1300px] min-w-[600px] w-full justify-start items-start mt-[45px] px-[50px]",
                div { class: "flex flex-row w-full justify-center items-center h-[60px] rounded-[10px] bg-white mb-[30px]",
                    if ctrl.get_select_page() == SelectPage::Summary {
                        div { class: "pr-[100px]",
                            div {
                                class: "flex flex-row w-[150px] justify-center items-center text-[#686868] font-semibold text-[22px] h-[45px] border-[2px] border-b-[#2168c3] border-r-transparent border-t-transparent border-l-transparent",
                                onclick: move |_| {
                                    ctrl.change_select_page(SelectPage::Summary);
                                },
                                {translates.survey_summary}
                            }
                        }
                        div {
                            class: "text-[#686868] font-semibold text-[22px]",
                            onclick: move |_| {
                                ctrl.change_select_page(SelectPage::Response);
                            },
                            {translates.individual_response}
                        }
                    } else {
                        div {
                            class: "text-[#686868] font-semibold text-[22px] pr-[100px]",
                            onclick: move |_| {
                                ctrl.change_select_page(SelectPage::Summary);
                            },
                            {translates.survey_summary}
                        }
                        div {
                            div {
                                class: "flex flex-row w-[150px] justify-center items-center text-[#686868] font-semibold text-[22px] h-[45px] border-[2px] border-b-[#2168c3] border-r-transparent border-t-transparent border-l-transparent",
                                onclick: move |_| {
                                    ctrl.change_select_page(SelectPage::Response);
                                },
                                {translates.individual_response}
                            }
                        }
                    }
                }
                div { class: "flex flex-col max-w-[1300px] min-w-[600px] w-full justify-start items-start mt-[10px] px-[30px] py-[20px] bg-white rounded-lg",
                    if ctrl.get_select_page() == SelectPage::Summary {
                        ResponseSummary {
                            response_report: translates.response_report,
                            response_download: translates.response_download,
                            response_summary: translates.response_summary,
                            total_number_of_responses: translates.total_number_of_responses,
                            completion_rate: translates.completion_rate,
                            normal_time_required: translates.normal_time_required,
                            most_skipped_questions: translates.most_skipped_questions,
                            response_attribute: translates.response_attribute,
                            survey_summary: translates.survey_summary,

                            number_of_replies: translates.number_of_replies,
                            number_of_skipped: translates.number_of_skipped,
                            item: translates.item,
                            reply: translates.reply,
                            total: translates.total,
                        }
                    } else {
                        ResponseList {
                            attribute_response: translates.attribute_response,
                            response_report: translates.response_report,
                            response_download: translates.response_download,
                            total_respondents: translates.total_respondents,
                            respondent_type: translates.respondent_type,
                            status: translates.status,
                            final_update_date: translates.final_update_date,
                            time_taken: translates.time_taken,
                            attribute: translates.attribute,
                            panel: translates.panel,
                            response_history: translates.response_history,
                            draft: translates.draft,
                            in_progress: translates.in_progress,
                            complete: translates.complete,
                        }
                    }
                }
            }
        }
    }
}
