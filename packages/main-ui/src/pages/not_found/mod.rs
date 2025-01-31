#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        div {
            div { class: "not_found flex flex-col justify-center items-center",
                img {
                    style: "padding-bottom: 40px; padding-top: 40px;",
                    src: asset!("/public/images/error.png"),
                    width: 50,
                    height: 50
                }
                div {
                    class: "text-3xl font-bold text-[#999999]",
                    style: "padding-bottom: 40px",
                    "Not Found"
                }
                div {
                    class: "text-xl font-normal text-[#999999]",
                    style: "padding-bottom: 40px",
                    "The Page you are looking for doesn't exists"
                }
            }
        }
    }
}
