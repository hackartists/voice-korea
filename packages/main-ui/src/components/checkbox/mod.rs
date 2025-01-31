#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    id: String,
    onchange: EventHandler<bool>,
    checked: Option<bool>,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        label {
            class: "{class} relative flex items-center rounded-full cursor-pointer",
            "for": "{id}",
            input {
                id: "{id}",
                "type": "checkbox",
                class: "peer relative h-[25px] w-[25px] cursor-pointer appearance-none rounded-[25px] border border-gray-900/20 bg-white transition-all before:absolute before:top-2/4 before:left-2/4 before:block before:h-[15px] before:w-[15px] before:-translate-y-2/4 before:-translate-x-2/4 before:rounded-[0px] before:bg-blue-gray-500 before:opacity-0 before:transition-opacity checked:border-[#2168c3] checked:bg-[#2168c3] checked:after:block checked:after:content-['✔'] checked:after:absolute checked:after:top-1/2 checked:after:left-1/2 checked:after:-translate-x-1/2 checked:after:-translate-y-1/2 checked:after:text-white checked:after:font-bold hover:scale-105 hover:before:opacity-0",
                checked: checked.unwrap_or(false),
                onchange: move |e| {
                    onchange(e.checked());
                },
            }
        }
    }
}
