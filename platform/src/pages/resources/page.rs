use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::resources::i18n::ResourceTranslate;

#[derive(Props, Clone, PartialEq)]
pub struct ResourceProps {
    lang: Language,
}

#[component]
pub fn ResourcePage(props: ResourceProps) -> Element {
    let translate: ResourceTranslate = translate(&props.lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translate.resource_title}"
            }
        }
        div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]", "{translate.resource_title}" }
        div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
            "{translate.resource_description}"
        }
    }
}
