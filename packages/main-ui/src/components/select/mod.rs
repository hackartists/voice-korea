#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub border: String,
    #[props(default = 0 as i64)]
    pub value: i64,
    #[props(default = "#000000".to_string())]
    pub color: String,
    #[props(default = "#ffffff".to_string())]
    pub background_color: String,
    pub onchange: Option<EventHandler<FormEvent>>,
    pub component: Element,
}

pub fn Select(props: SelectProps) -> Element {
    rsx! {
        select {
            class: props.class,
            border: props.border,
            value: props.value,
            color: props.color,
            background_color: props.background_color,
            onchange: move |event| {
                if let Some(onchange) = props.onchange {
                    onchange(event)
                }
            },
            {props.component}
        }
    }
}
