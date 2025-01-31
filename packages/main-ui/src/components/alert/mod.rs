use dioxus::prelude::*;

#[component]
pub fn AlertModal(
    children: Element,
    tail: Element,
    width: Option<u64>,
    max_width: Option<u64>,
) -> Element {
    let width: u64 = width.unwrap_or(460);
    rsx! {
        div {
            class: "absolute flex flex-row w-screen h-screen backdrop-blur-sm justify-center items-center z-50",
            div {
                class: "relative flex flex-col h-min max-h-[750px] bg-white rounded-lg border-[1px] border-[#9f9f9f] justify-start items-center overflow-y-auto",
                style: if let Some(max_width) = max_width {
                    format!("top: -120px; left: -250px; padding-left: 30px; padding-right: 30px; padding-top: 20px; padding-bottom: 20px; min-width: {}px; max-width: {}px", width, max_width)
                } else {
                    format!("top: -120px; left: -250px; padding-left: 30px; padding-right: 30px; padding-top: 20px; padding-bottom: 20px; width: {}px", width)
                },
                {children}
                div {
                    class: "flex flex-row w-full justify-end items-end p-4 mt-[50px]",
                    {tail}
                }
            }
        }
    }
}
