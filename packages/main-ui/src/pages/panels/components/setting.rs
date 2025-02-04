use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::panels::i18n::PanelTranslate;

#[component]
pub fn AttributeSetting(
    lang: Language,
    name: String,
    total_options: Vec<String>,
    current_option: String,

    onsave: EventHandler<String>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let translate: PanelTranslate = translate(&lang);
    let mut selected = use_signal(|| current_option);
    rsx! {
        div { class: "flex flex-col w-[400px] justify-start",
            div { class: "flex flex-col w-full max-h-[350px] justify-start items-start overflow-y-auto mb-[20px]",
                for option in total_options {
                    label { class: "flex flex-row w-full justify-start items-center cursor-pointer mb-[20px] gap-[20px]",
                        input {
                            r#type: "radio",
                            name: name.clone(),
                            value: option.clone(),
                            checked: selected() == option.clone(),
                            class: "hidden",
                            oninput: {
                                let option = option.clone();
                                move |_| selected.set(option.clone())
                            },
                        }
                        div {
                            class: format!(
                                "w-[22px] h-[20px] border-2 border-gray-400 rounded-full flex flex-row items-center justify-center {} p-[2px]",
                                if selected() == option.clone() { "border-blue-500" } else { "" },
                            ),
                            div {
                                class: format!(
                                    "w-full h-full bg-blue-500 rounded-full {}",
                                    if selected() == option.clone() { "" } else { "hidden" },
                                ),
                            }
                        }
                        span { class: "flex flex-row w-full font-normal text-black text-[16px] leading-[24.2px]",
                            "{option}"
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-center",
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px] leading-[24px] mr-[20px]",
                    onclick: move |_| {
                        onsave.call(selected());
                    },
                    "{translate.save}"
                }
                button {
                    class: "flex flex-row px-[14px] py-[8px] bg-white font-semibold text-[16px] text-[#222222]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}
