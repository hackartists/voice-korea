#![allow(non_snake_case)]
use crate::{components::table_row::Row, pages::create::controller::Controller, prelude::*};
use dioxus::prelude::*;

use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct StepFourProps {
    lang: Language,
    complete_join_membership_info: String,
    email_address: String,
    name_info: String,
    phone_info: String,
    company_name_info: String,
    complete: String,
    company_name_example: String,
}

#[component]
pub fn StepFourPage(props: StepFourProps) -> Element {
    let ctrl: Controller = use_context();
    let navigator = use_navigator();
    rsx! {
        div { class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div { class: "text-[32px] font-bold text-black pb-[30px]",
                "{props.complete_join_membership_info}"
            }
            div { class: "flex flex-col w-full h-full items-start justify-center pb-[340px]",
                Row {
                    enable_bottom_border: true,
                    label: props.email_address,
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
                            div { class: "text-[16px] font-normal text-black", {ctrl.get_email_address()} }
                        }
                    },
                }
                        // Row {
            //     enable_bottom_border: false,
            //     label: props.name_info,
            //     element: rsx! {
            //         div {
            //             class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
            //             div {
            //                 class: "text-[16px] font-normal text-black",
            //                 {ctrl.get_name()}
            //             }
            //         }
            //     }
            // }
            // Row {
            //     enable_bottom_border: false,
            //     label: props.phone_info,
            //     element: rsx! {
            //         div {
            //             class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
            //             div {
            //                 class: "text-[16px] font-normal text-black",
            //                 {ctrl.get_cellphone_number()}
            //             }
            //         }
            //     }
            // }
            // Row {
            //     enable_bottom_border: true,
            //     label: props.company_name_info,
            //     element: rsx! {
            //         div {
            //             class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
            //             div {
            //                 class: "text-[16px] font-normal text-black",
            //                 {props.company_name_example}
            //             }
            //         }
            //     }
            // }
            }
            div { class: "flex flex-row w-full justify-end min-w-[710px] items-end pt-[30px]",
                button { class: "flex flex-row w-[300px] h-[60px] justify-end items-end bg-[#f5f5f5] border-solid border border-[#e0e0e0]",
                    div {
                        class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-black",
                        onclick: move |_| {
                            navigator
                                .push(Route::LoginPage {
                                    lang: props.lang.clone(),
                                });
                        },
                        {props.complete}
                    }
                }
            }
        }
    }
}
