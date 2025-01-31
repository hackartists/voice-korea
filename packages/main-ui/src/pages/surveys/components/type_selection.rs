use dioxus::prelude::*;

#[component]
pub fn SurveyTypeSelectionBox(
    selected_type: String,
    types: Vec<String>,
    change_type: EventHandler<String>,
) -> Element {
    rsx! {
        select {
            class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px] font-medium text-[15px] text-[#b4b4b4]",
            value: selected_type.clone(),
            onchange: move |e: Event<FormData>| {
                change_type.call(e.value());
            },
            option { value: "", disabled: true, selected: selected_type == "", "형식 선택" }
            for survey_type in types {
                option {
                    value: survey_type.clone(),
                    selected: selected_type == survey_type,
                    "{survey_type}"
                }
            }
        }
    }
}
