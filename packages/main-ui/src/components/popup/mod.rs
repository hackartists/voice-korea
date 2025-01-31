#![allow(non_snake_case)]
use crate::components::icons::Cancel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PopupProps {
    #[props(default = "".to_string())]
    title: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = true)]
    default_header: bool,
    style: Option<String>,
    children: Element,
    is_open: bool,
    onclose: EventHandler<MouseEvent>,
}

#[component]
pub fn Popup(props: PopupProps) -> Element {
    let handle_close = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        props.onclose.call(evt);
    };
    client! {
        if props.is_open {
            dioxus::document::eval(
                r#"document.body.classList.add("overflow-hidden");"#,
            );
        } else {
            dioxus::document::eval(r#"document.body.classList.remove("overflow-hidden");"#);
        }
    }
    let visible = if props.is_open { "flex" } else { "hidden" };
    rsx! {
        div {
            class: "fixed top-0 left-0 w-full h-full backdrop-blur-sm lg:backdrop-blur-lg {visible} justify-center items-center",
            style: "z-index: 21;",
            onclick: handle_close,
            div {
                class: "{props.class} bg-white rounded-xl shadow-lg p-7",
                style: match props.style {
                    Some(style) => style,
                    None => "".to_string(),
                },
                onclick: move |evt| { evt.stop_propagation() },
                if props.default_header {
                    div { class: "flex justify-center w-full font-semibold text-xl items-center",
                        div { class: "flex-[0_0_1.25rem]" }
                        h1 { class: "flex-1 text-center", {props.title} }
                        div { class: "flex-[0_0_1.25rem]", onclick: handle_close, Cancel {} }
                    }
                }
                {props.children}
            }
        }
    }
}
