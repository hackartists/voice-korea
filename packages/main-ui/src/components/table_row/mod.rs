#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Row(
    #[props(default = false)] enable_bottom_border: bool,
    #[props(default = 70)] height: u64,
    label: String,
    element: Element,
) -> Element {
    let bottom_border = if enable_bottom_border {
        "border-b-[#e0e0e0]"
    } else {
        "border-b-[#ffffff]"
    };

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
                div {
                    class: "flex flex-row w-full min-w-[710px] border-solid border border-t-[#e0e0e0] {bottom_border} border-l-[#e0e0e0] border-r-[#ffffff]",
                    style: "height: {height}px",
                    div {
                        class: "flex flex-row w-[200px] min-w-[200px] h-full justify-start items-start bg-[#2168c3]",
                        div {
                            class: "min-w-[200px] p-[20px] text-white text-[16px] font-normal",
                            "{label}"
                        }
                    }
                    {element}
                }
        }
    }
}
