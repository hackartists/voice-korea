#![allow(non_snake_case)]
use crate::{pages::create::controller::Controller, prelude::*};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct StepOneProps {
    lang: Language,
    authorization: String,
    individual: String,
    company: String,
    individual_description: String,
    phone: String,
    phone_description: String,
    check_title: String,
    check_descriptions: Vec<String>,
    company_name: String,
    business_register_number: String,
    company_name_example: String,
    business_register_number_example: String,
    next: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct PersonalProps {
    lang: Language,
    individual_description: String,
    phone: String,
    phone_description: String,
    check_title: String,
    check_descriptions: Vec<String>,
}

#[derive(PartialEq, Props, Clone)]
pub struct CompanyProps {
    lang: Language,
    company_name: String,
    business_register_number: String,
    company_name_example: String,
    business_register_number_example: String,
    next: String,
}

#[component]
pub fn StepOnePage(props: StepOneProps) -> Element {
    let mut ctrl: Controller = use_context();

    rsx! {
        div { class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div { class: "text-[32px] font-bold text-black pb-[20px]", "{props.authorization}" }
            div { class: "flex flex-row w-full justify-center items-center pb-[40px]",
                div {
                    class: if ctrl.get_authorize_type() == 1 { "flex flex-row h-[45px]" } else { "flex flex-row h-[45px] bg-[#2168c3]" },
                    style: if ctrl.get_authorize_type() == 1 { "width: calc(100vh); border: 1px solid #e0e0e0; color: #ffffff;" } else { "width: calc(100vh)" },
                    onclick: move |_| {
                        ctrl.set_authorize_type(0);
                    },
                    div { class: if ctrl.get_authorize_type() == 1 { "flex flex-row w-full h-full justify-center items-center text-[20px] font-normal text-black" } else { "flex flex-row w-full h-full justify-center items-center text-[20px] font-normal text-white" },
                        "{props.individual}"
                    }
                }
                div {
                    class: if ctrl.get_authorize_type() == 0 { "flex flex-row h-[45px]" } else { "flex flex-row h-[45px] bg-[#2168c3]" },
                    style: if ctrl.get_authorize_type() == 0 { "width: calc(100vh); border: 1px solid #e0e0e0; color: #ffffff;" } else { "width: calc(100vh)" },
                    onclick: move |_| {
                        ctrl.set_authorize_type(1);
                    },
                    div { class: if ctrl.get_authorize_type() == 0 { "flex flex-row w-full h-full justify-center items-center text-[20px] font-normal text-black" } else { "flex flex-row w-full h-full justify-center items-center text-[20px] font-normal text-white" },
                        "{props.company}"
                    }
                }
            }
            if ctrl.get_authorize_type() == 0 {
                PersonalAuthorize {
                    lang: props.lang,
                    individual_description: props.individual_description,
                    phone: props.phone,
                    phone_description: props.phone_description,
                    check_title: props.check_title,
                    check_descriptions: vec![
                        props.check_descriptions[0].clone(),
                        props.check_descriptions[1].clone(),
                        props.check_descriptions[2].clone(),
                    ],
                }
            } else {
                CompanyAuthorize {
                    lang: props.lang,
                    company_name: props.company_name,
                    business_register_number: props.business_register_number,
                    company_name_example: props.company_name_example,
                    business_register_number_example: props.business_register_number_example,
                    next: props.next,
                }
            }
        
        }
    }
}

#[component]
pub fn CompanyAuthorize(props: CompanyProps) -> Element {
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col justify-start items-start w-full h-full",
            div { class: "flex flex-row w-full text-[#636363] font-normal text-[24px] pb-[40px]",
                "Blah blah Blah blah Blah blah Blah blah Blah blah Blah blah Blah blah Blah blah Blah blah Blah blah"
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full h-[60px] border-solid border border-t-[#e0e0e0] border-b-[#e0e0e0] border-l-[#e0e0e0] border-r-[#ffffff]",
                    div { class: "flex flex-row w-[200px] h-full justify-start items-start bg-[#2168c3]",
                        div { class: "p-[20px] text-white text-[16px] font-normal",
                            "{props.company_name}"
                        }
                    }
                    div { class: "flex flex-row w-[300px] h-full justify-start items-start p-[10px]",
                        input {
                            class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                            r#type: "text",
                            style: "border: 1px solid #e0e0e0; color: #8e929b;",
                            placeholder: "{props.company_name_example}",
                            value: ctrl.get_company(),
                            oninput: move |event| ctrl.set_company(event.value()),
                        }
                    }
                }
                div { class: "flex flex-row w-full h-[60px] border-solid border border-t-transparent border-b-[#e0e0e0] border-l-[#e0e0e0] border-r-[#ffffff]",
                    div { class: "flex flex-row w-[200px] h-full justify-start items-start bg-[#2168c3]",
                        div { class: "p-[20px] text-white text-[16px] font-normal",
                            "{props.business_register_number}"
                        }
                    }
                    div { class: "flex flex-row w-[300px] h-full justify-start items-start p-[10px]",
                        input {
                            class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                            r#type: "text",
                            style: "border: 1px solid #e0e0e0; color: #8e929b;",
                            placeholder: "{props.business_register_number_example}",
                            value: ctrl.get_business_number(),
                            oninput: move |event| ctrl.set_business_number(event.value()),
                        }
                    }
                }
                div { class: "flex flex-row w-full justify-end items-end pt-[285px]",
                    div {
                        class: "flex flex-row w-[300px] h-[60px] justify-end items-end bg-[#2168c3]",
                        onclick: move |_| ctrl.set_step(1),
                        div { class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-white",
                            "{props.next}"
                        }
                    }
                }
            }
        

        }
    }
}

#[component]
pub fn PersonalAuthorize(props: PersonalProps) -> Element {
    rsx! {
        div { class: "flex flex-col justify-start items-start w-full h-full",
            div { class: "flex flex-row w-full text-[#636363] font-normal text-[24px] pb-[40px]",
                "{props.individual_description}"
            }
            div { class: "flex flex-row w-full h-full justify-center items-center pb-[170px]",
                div { class: "flex flex-row justify-center items-center w-[520px] h-[130px] bg-[#f2f2f2]",
                    div { class: "flex flex-row w-full h-full justify-start items-center px-[20px] py-[30px]",
                        img {
                            src: asset!("/public/images/phone.png"),
                            alt: "Cellphone",
                        }
                        div { class: "flex flex-col w-full h-full justify-start items-start pr-[10px]",
                            div { class: "text-[#636363] font-bold text-[24px] pb-[10px]",
                                "{props.phone}"
                            }
                            div { class: "text-[#636363] font-normal text-[16px]",
                                "{props.phone_description}"
                            }
                        }
                        img {
                            src: asset!("/public/images/right-arrow.png"),
                            alt: "Right Arrow",
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full h-min border-solid border border-[#e0e0e0] px-[20px] py-[15px]",
                div { class: "text-black text-[20px] font-normal pb-[20px]", "{props.check_title}" }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_descriptions[0]}"
                }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_descriptions[1]}"
                }
                div { class: "text-[#636363] text-[20px] font-normal", "{props.check_descriptions[2]}" }
            }
        }
    }
}
