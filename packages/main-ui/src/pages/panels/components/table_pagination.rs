use dioxus::prelude::*;

use crate::components::icons::{ArrowLeft, ArrowRight};

#[component]
pub fn TablePagination(
    onprev: EventHandler<usize>,
    onnext: EventHandler<usize>,
    #[props(default = 10)] size: usize,
    #[props(default = 10)] maximum_size: usize,
    page: usize,
) -> Element {
    rsx! {
        div { class: "flex flex-row gap-[10px]",
            div {
                class: format!(
                    "w-[25px] h-[25px] {}",
                    if page == 1 { "cursor-not-allowed" } else { "cursor-pointer" },
                ),
                onclick: move |_| {
                    if page != 1 {
                        onprev.call(page - 1);
                    }
                },
                ArrowLeft { width: "25", height: "25", color: "#555462" }
            }
            div {
                class: format!(
                    "w-[25px] h-[25px] {}",
                    if size < maximum_size { "cursor-not-allowed" } else { "cursor-pointer" },
                ),
                onclick: move |_| {
                    if size >= maximum_size {
                        onnext.call(page + 1);
                    }
                },
                ArrowRight { width: "25", height: "25", color: "#555462" }
            }
        }
    }
}
