#![allow(non_snake_case)]
mod i18n;

use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::BottomTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct BottomProps {
    lang: Language,
}

#[component]
pub fn Bottom(props: BottomProps) -> Element {
    let translates: BottomTranslate = translate(&props.lang);

    let address = translates.address;
    let company = translates.company;
    let company_address = translates.company_address;
    let ceo = translates.ceo;
    let register_address = translates.register_address;
    let copyright = translates.copyright;

    let bottom_text_style = "text-[14px] font-normal text-white";
    rsx! {
        div { class: "flex flex-row w-full items-start justify-start h-[135px] bg-[#2168C3]",
            div { class: "flex flex-col h-full py-[20px] pl-[50px] pr-[26px]",
                div { class: "flex flex-col w-max h-min justify-center items-center",
                    img {
                        class: "flex flex-col pb-[20px]",
                        src: asset!("/public/images/logo-white.png"),
                        width: 40,
                        height: 40,
                        alt: "Voice Korea Logo",
                    }
                    div { class: "flex flex-row text-[16px] font-bold text-white", "VOICE KOREA" }
                }
            }
            div { class: "flex flex-col w-full h-full",
                div { class: "flex flex-col w-full h-full px-[10px] py-[25px]",
                    div { class: "{bottom_text_style} mb-[5px]", "{address}" }
                    div { class: "flex flex-row w-full justify-start items-start mb-[5px]",
                        div { class: "{bottom_text_style} pr-[20px]", "{company}" }
                        div { class: "{bottom_text_style} pr-[20px]", "{company_address}" }
                        div { class: "{bottom_text_style} pr-[20px]", "{ceo}" }
                        div { class: "{bottom_text_style}", "{register_address}" }
                    }

                    div { class: "text-[12px] font-normal text-white", "{copyright}" }
                }
            }
        }
    }
}
