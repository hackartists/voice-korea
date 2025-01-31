#![allow(non_snake_case)]
use crate::{
    components::{input::Input, table_row::Row},
    prelude::*,
};
use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::FindEmailTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct FindEmailPageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct FindEmailProps {
    lang: Language,
    ctrl: controller::Controller,
    name_label: String,
    name_hint: String,
    phone_label: String,
    phone_hint: String,
    send_authentication_number: String,
    authentication_number: String,
    authentication_number_description: Vec<String>,
    check_title: String,
    check_description: Vec<String>,
    find_email: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct GetEmailProps {
    lang: Language,
    ctrl: controller::Controller,
    get_email_description: String,
    email_address: String,
    go_to_login: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct SendAuthenticationButtonProps {
    width: Option<i64>,
    label: String,
    lang: Language,
    onclick: EventHandler<MouseEvent>,
}

pub mod controller;
pub mod i18n;

#[component]
pub fn FindEmailPage(props: FindEmailPageProps) -> Element {
    let ctrl = controller::Controller::init();
    let translates: FindEmailTranslate = translate(&props.lang.clone());

    rsx! {
        div { class: "flex flex-col w-full h-full items-start justify-center px-[170px] py-[85px]",
            div { class: "text-[32px] font-bold text-black pb-[30px]", "{translates.find_email}" }
            if ctrl.get_email_address() == "" {
                FindEmail {
                    lang: props.lang.clone(),
                    ctrl,
                    name_label: translates.name_label,
                    name_hint: translates.name_hint,
                    phone_label: translates.phone_label,
                    phone_hint: translates.phone_hint,
                    send_authentication_number: translates.send_authentication_number,
                    authentication_number: translates.authentication_number,
                    authentication_number_description: vec![
                        translates.authentication_number_description_0.to_string(),
                        translates.authentication_number_description_1.to_string(),
                    ],
                    check_title: translates.check_title,
                    check_description: vec![
                        translates.check_description_0.to_string(),
                        translates.check_description_1.to_string(),
                    ],
                    find_email: translates.find_email,
                }
            } else {
                GetEmail {
                    lang: props.lang.clone(),
                    ctrl,
                    get_email_description: translates.get_email_description,
                    email_address: translates.email_address,
                    go_to_login: translates.go_to_login,
                }
            }

        }
    }
}

#[component]
pub fn GetEmail(props: GetEmailProps) -> Element {
    let email = props.ctrl.get_email_address();
    rsx! {
        div { class: "flex flex-col w-full h-full",
            div { class: "flex flex-col w-full h-full items-start justify-start pb-[30px]",
                div { class: "text-[#636363] font-normal text-[24px] mb-[30px]",
                    {props.get_email_description}
                }
                div { class: "flex flex-col w-full justify-start items-start mb-[490px]",
                    Row {
                        enable_bottom_border: true,
                        label: props.email_address,
                        element: rsx! {
                            div { class: "flex flex-row w-full h-full items-center mx-[20px] text-black font-normal text-[16px]",
                                "{email}"
                            }
                        },
                    }
                }
                div { class: "flex flex-row w-full justify-end items-end",
                    Link {
                        to: Route::LoginPage {
                            lang: props.lang.clone(),
                        },
                        div { class: "flex flex-row w-[300px] h-[60px] bg-[#2168c3] justify-center items-center text-white font-bold text-[24px]",
                            "{props.go_to_login}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FindEmail(props: FindEmailProps) -> Element {
    let mut ctrl = props.ctrl.clone();
    rsx! {
        div { class: "flex flex-col w-full h-full",
            div { class: "flex flex-col w-full h-full items-start justify-center pb-[30px]",
                Row {
                    enable_bottom_border: false,
                    label: props.name_label,
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center",
                            div { class: "mx-[10px]",
                                Input {
                                    value: ctrl.get_name(),
                                    placeholder: props.name_hint,
                                    onchange: move |e| {
                                        ctrl.set_name(e);
                                    },
                                }
                            }
                        }
                    },
                }
                Row {
                    enable_bottom_border: false,
                    label: props.phone_label,
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center",
                            div { class: "mx-[10px]",
                                Input {
                                    value: ctrl.get_phone_number(),
                                    placeholder: props.phone_hint,
                                    onchange: move |e| {
                                        ctrl.set_phone_number(e);
                                    },
                                }
                            }
                            SendAuthenticationButton {
                                label: props.send_authentication_number,
                                lang: props.lang,
                                onclick: move |_| {
                                    ctrl.set_click_send_authentication();
                                },
                            }
                        }
                    },
                }
                Row {
                    enable_bottom_border: true,
                    height: 135,
                    label: props.authentication_number,
                    element: rsx! {
                        div { class: "flex flex-col w-full h-full justify-start items-start mt-[10px] ml-[10px]",
                            div { class: "pb-[5px]",
                                Input {
                                    value: ctrl.get_authentication_number(),
                                    onchange: move |e| {
                                        ctrl.set_authentication_number(e);
                                    },
                                }
                            }
                            div { class: "text-[16px] font-normal text-[#636363]",
                                "{props.authentication_number_description[0]}"
                            }
                            div { class: "text-[16px] font-normal text-[#636363]",
                                "{props.authentication_number_description[1]}"
                            }
                        }
                    },
                }
            }
            div { class: "flex flex-col w-full h-min min-w-[710px] border-solid border border-[#e0e0e0] px-[20px] py-[30px] my-[30px]",
                div { class: "text-black text-[20px] font-normal pb-[20px]", "{props.check_title}" }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_description[0]}"
                }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_description[1]}"
                }
            }
            div { class: "flex flex-row w-full justify-end items-end",
                div {
                    class: "flex flex-row w-[300px] h-[60px] bg-[#2168c3] justify-center items-center text-white font-bold text-[24px]",
                    onclick: move |_| {
                        ctrl.find_email_address();
                    },
                    "{props.find_email}"
                }
            }
        }
    }
}

#[component]
pub fn SendAuthenticationButton(props: SendAuthenticationButtonProps) -> Element {
    let width = match props.width {
        Some(w) => format!("w-[{}px]", w),
        None => {
            if props.lang == Language::En {
                "w-[270px]".to_string()
            } else {
                "w-[135px]".to_string()
            }
        }
    };

    rsx! {
        div {
            class: format!("flex flex-col {} h-[35px] justify-start items-start", width),
            onclick: move |evt| {
                props.onclick.call(evt);
            },
            style: "border: 1px solid; border-color: rgba(33, 104, 195, 0.5); border-radius: 5px; background-clip: padding-box; background-color: rgba(33, 104, 195, 0.04);",
            div { class: "flex flex-row justify-center items-center w-full h-full text-[#2168c3] font-normal text-[15px]",
                {props.label}
            }
        }
    }
}
