#![allow(non_snake_case)]
use crate::{pages::create::controller::Controller, prelude::*};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct StepTwoProps {
    lang: Language,
    agree_terms: String,
    agree_membership_terms: String,
    agree_privacy_policy: String,
    entrust_personal_information: String,
    essential: String,
}

#[component]
pub fn StepTwoPage(props: StepTwoProps) -> Element {
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div { class: "text-[32px] font-bold text-black pb-[50px]", "{props.agree_terms}" }
            div { class: "flex flex-col w-full justify-start items-start pb-[50px]",
                div { class: "flex flex-row w-full h-[50px]",
                    div {
                        class: "flex flex-row w-full h-[50px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full justify-between items-center px-[20px] py-[10px]",
                            div {
                                class: "flex flex-row h-full justify-center items-center",
                                onclick: move |_| ctrl.set_extend_first_terms(!ctrl.get_extend_first()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_extend_first() { "/images/bottom-polygon.png" } else { "/images/right-polygon.png" },
                                    alt: "arrow",
                                }
                                div { class: "text-[24px] font-bold text-black pr-[10px]",
                                    "{props.agree_membership_terms}"
                                }
                                div { class: "text-[24px] font-bold text-[#1d18ff]",
                                    "{props.essential}"
                                }
                            }
                            div { onclick: move |_| ctrl.set_click_first_terms(!ctrl.get_click_first()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_click_first() { "/images/enable_check.png" } else { "/images/not_enable_check.png" },
                                    alt: "check",
                                }
                            }
                        }
                    }
                }
                if ctrl.get_extend_first() {
                    div {
                        class: "flex flex-col w-full h-[200px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full px-[20px] py-[20px] text-[16px] font-normal text-black whitespace-pre-line",
                            "제 1장 총칙
                            제 1조 목적
                            이 약관은 IITP의 이용에 관한 조선 및 전차와 기타 필요한 사항을 규정하는 것을 목적으로 합니다.
                            BLAH BLAH"
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start pb-[50px]",
                div { class: "flex flex-row w-full h-[50px]",
                    div {
                        class: "flex flex-row w-full h-[50px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full justify-between items-center px-[20px] py-[10px]",
                            div {
                                class: "flex flex-row h-full justify-center items-center",
                                onclick: move |_| ctrl.set_extend_second_terms(!ctrl.get_extend_second()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_extend_second() { "/images/bottom-polygon.png" } else { "/images/right-polygon.png" },
                                    alt: "right-polygon",
                                }
                                div { class: "text-[24px] font-bold text-black pr-[10px]",
                                    "{props.agree_privacy_policy}"
                                }
                                div { class: "text-[24px] font-bold text-[#1d18ff]",
                                    "{props.essential}"
                                }
                            }
                            div { onclick: move |_| ctrl.set_click_second_terms(!ctrl.get_click_second()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_click_second() { "/images/enable_check.png" } else { "/images/not_enable_check.png" },
                                    alt: "check",
                                }
                            }
                        }
                    }
                }
                if ctrl.get_extend_second() {
                    div {
                        class: "flex flex-col w-full h-[200px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full px-[20px] py-[20px] text-[16px] font-normal text-black whitespace-pre-line",
                            "제 1장 총칙
                            제 1조 목적
                            이 약관은 IITP의 이용에 관한 조선 및 전차와 기타 필요한 사항을 규정하는 것을 목적으로 합니다.
                            BLAH BLAH"
                        }
                    }
                }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full h-[50px]",
                    div {
                        class: "flex flex-row w-full h-[50px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full justify-between items-center px-[20px] py-[10px]",
                            div {
                                class: "flex flex-row h-full justify-center items-center",
                                onclick: move |_| ctrl.set_extend_third_terms(!ctrl.get_extend_third()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_extend_third() { "/images/bottom-polygon.png" } else { "/images/right-polygon.png" },
                                    alt: "right-polygon",
                                }
                                div { class: "text-[24px] font-bold text-black pr-[10px]",
                                    "{props.entrust_personal_information}"
                                }
                                div { class: "text-[24px] font-bold text-[#1d18ff]",
                                    "{props.essential}"
                                }
                            }
                            div { onclick: move |_| ctrl.set_click_third_terms(!ctrl.get_click_third()),
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_click_third() { "/images/enable_check.png" } else { "/images/not_enable_check.png" },
                                    alt: "check",
                                }
                            }
                        }
                    }
                }
                if ctrl.get_extend_third() {
                    div {
                        class: "flex flex-col w-full h-[200px] justify-between items-center",
                        style: "border: 1px solid #e0e0e0; color: #ffffff;",
                        div { class: "flex flex-row w-full h-full px-[20px] py-[20px] text-[16px] font-normal text-black whitespace-pre-line",
                            "제 1장 총칙
                            제 1조 목적
                            이 약관은 IITP의 이용에 관한 조선 및 전차와 기타 필요한 사항을 규정하는 것을 목적으로 합니다.
                            BLAH BLAH"
                        }
                    }
                }
            }
        }
    }
}
