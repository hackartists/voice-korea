#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::id::response_report::controller::{use_controller, ResponseStatus, ResponseType};

#[derive(Props, Clone, PartialEq)]
pub struct ResponseListProps {
    attribute_response: String,
    response_report: String,
    response_download: String,
    total_respondents: String,
    respondent_type: String,
    status: String,
    final_update_date: String,
    time_taken: String,
    attribute: String,
    panel: String,
    response_history: String,
    draft: String,
    in_progress: String,
    complete: String,
}

pub fn ResponseList(props: ResponseListProps) -> Element {
    let mut ctrl = use_controller();
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
            div {
                class: "text-[#2168c3] font-semibold text-[30px]",
                "{props.response_report}"
            }
            div {
                class: "flex flex-row w-full justify-end items-end mb-[20px]",
                div {
                    class: "flex flex-row w-[135px] h-[45px] justify-center items-center mt-[30px] rounded-lg bg-[#1e5eaf] text-white font-bold text-[16px]",
                    "{props.response_download}"
                }
            }
            div {
                class: "px-[35px] mb-[20px]",
                div {
                    class: "text-black font-normal text-[16px]",
                    {format!("{} {}명", props.total_respondents, ctrl.get_panels().len())}
                }
            }
            div {
                class: "flex flex-row w-full h-[35px] text-[16px] font-normal text-[#696969]",
                div {
                    class: "w-[105px]",
                }
                div {
                    class: "flex flex-row justify-center items-center w-[90px]",
                    "{props.respondent_type}"
                }
                div {
                    class: "flex flex-row justify-center items-center w-[150px]",
                    "{props.final_update_date}"
                }
                div {
                    class: "flex flex-row justify-center items-center w-[85px]",
                    "{props.status}"
                }
                div {
                    class: "flex flex-row justify-center items-center w-[100px]",
                    "{props.time_taken}"
                }
                div {
                    class: "flex flex-row justify-center items-center w-[130px]",
                    {format!("{}1", props.attribute.clone())}
                }
                div {
                    class: "flex flex-row justify-center items-center w-[100px]",
                    {format!("{}2", props.attribute.clone())}
                }
                div {
                    class: "flex flex-row justify-center items-center w-[100px]",
                    {format!("{}3", props.attribute.clone())}
                }
                div {
                    class: "flex flex-row justify-center items-center w-[100px]",
                    {format!("{}4", props.attribute.clone())}
                }
                div {
                    class: "flex flex-row justify-center items-center w-[100px]",
                    "..."
                }
            }
            for (i, panel) in ctrl.get_panels().iter().enumerate() {
                div {
                    class: "flex flex-row w-full h-[45px] justify-start items-center",
                    div {
                        class: if i % 2 == 0 {"flex flex-row w-full h-[45px] justify-start items-center bg-[#f9f9f9] text-black font-normal text-[14px]"} else {"flex flex-row w-full h-[45px] justify-start items-center bg-white text-black font-normal text-[14px]"},
                        div {
                            class: "flex flex-row justify-center items-center w-[105px] text-black font-semibold text-[16px]",
                            {format!("{} {}", props.panel, i + 1)}
                        }
                        if let ResponseType::AttributeResponse = panel.response_type {
                            div {
                                class: "flex flex-row justify-center items-center w-[90px]",
                                "{props.attribute_response}"
                            }
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[150px]",
                            "{panel.final_update_date}"
                        }
                        match panel.response_status {
                            Some(status) => {
                                rsx! {
                                    if let ResponseStatus::NotProgress = status {
                                        div {
                                            class: "flex flex-row justify-center items-center w-[85px] text-[#2168c3] font-semibold",
                                            "{props.draft}"
                                        }
                                    } else if let ResponseStatus::InProgress = status {
                                        div {
                                            class: "flex flex-row justify-center items-center w-[85px] text-[#2168c3] font-semibold",
                                            "{props.in_progress}"
                                        }
                                    } else {
                                        div {
                                            class: "flex flex-row justify-center items-center w-[85px] text-[#2168c3] font-semibold",
                                            "{props.complete}"
                                        }
                                    }
                                }
                            },
                            None => {
                                rsx! {
                                    div {
                                        class: "flex flex-row justify-center items-center w-[85px] text-black font-semibold",
                                        "-"
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[100px] text-black font-semibold",
                            "-"
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[130px]",
                            {panel.attribute.get(0).unwrap().clone()}
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[100px]",
                            {panel.attribute.get(1).unwrap().clone()}
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[100px]",
                            {panel.attribute.get(2).unwrap().clone()}
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[100px]",
                            {panel.attribute.get(3).unwrap().clone()}
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[100px]",
                            "..."
                        }
                    }
                    if ctrl.get_clicked_index() != i {
                        div {
                            class: "ml-[10px] flex flex-row w-[100px] h-[30px] justify-center items-center border-[2px] rounded-xl bg-white border-[#9f9f9f]",
                            onclick: move |_| {
                                ctrl.change_clicked_index(i);
                            },
                            div {
                                class: "font-medium text-[#1e5eaf] text-[14px]",
                                "{props.response_history}"
                            }
                        }
                    } else {
                        div {
                            class: "ml-[10px] flex flex-row w-[100px] h-[30px] justify-center items-center border-[2px] rounded-xl bg-[#1e5eaf]",
                            onclick: move |_| {
                                ctrl.change_clicked_index(i);
                            },
                            div {
                                class: "font-medium text-white text-[14px]",
                                "{props.response_history}"
                            }
                        }
                    }
                }
            }
        }
    }
}
