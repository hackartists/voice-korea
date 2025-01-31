#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub button_text: String,
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub text_class: String,
}

pub fn Button(props: ButtonProps) -> Element {
    const DEFAULT_BUTTON_STYLES: &str = "rounded-xl justify-center items-center";
    const DEFAULT_TEXT_STYLES: &str = "text-xl font-normal text-white whitespace-nowrap";
    rsx! {
        button {
            class: "{props.class} {DEFAULT_BUTTON_STYLES}",
            onclick: move |event| {
                if let Some(onclick) = props.onclick {
                    onclick(event)
                }
            },
            div { class: "{props.text_class} {DEFAULT_TEXT_STYLES}",
                "{props.button_text}"
            }
        }
    }
}
