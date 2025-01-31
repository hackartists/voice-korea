#![allow(non_snake_case)]
use crate::{
    components::{
        alert::AlertModal,
        icons::{Add, Close, Search},
    },
    pages::id::select_response::response_type::controller::{
        self, AttributeModel, SelectAttribute,
    },
    prelude::*,
};
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct SelectAttributeProps {
    lang: Language,
    survey_id: String,
    select_type: String,

    temporary_save: String,
    attribute_title: String,
    attribute_description: String,
    attribute_select_label: String,
    nation: String,
    gender: String,
    age: String,
    add_attribute: String,
    cancel: String,
    save: String,

    search_result: String,
    selected_attribute: String,
    search_hint: String,

    attribute_setting: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct ModalTranslates {
    search_hint: String,
    search_result: String,
    add_attribute: String,
    selected_attribute: String,
    cancel: String,
    save: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct SettingModalTranslates {
    attribute_setting: String,
    cancel: String,
    save: String,
}

#[component]
pub fn SelectAttributePage(props: SelectAttributeProps) -> Element {
    let steps = vec![0, 50, 100, 250, 500, 1000, 3000, 5000];
    let mut ctrl = controller::use_controller();

    rsx! {
        div { class: "flex flex-col",
            if ctrl.get_clicked_add_attribute() {
                AttributeAddModal {
                    translates: ModalTranslates {
                        search_hint: props.search_hint.clone(),
                        search_result: props.search_result.clone(),
                        add_attribute: props.add_attribute.clone(),
                        selected_attribute: props.selected_attribute.clone(),
                        cancel: props.cancel.clone(),
                        save: props.save.clone(),
                    },
                    clicked_cancel_button: move |_e: MouseEvent| {
                        ctrl.clicked_attribute_modal_cancel_button();
                    },
                    clicked_add_button: move |_e: MouseEvent| {
                        ctrl.clicked_add_button();
                    },
                    clicked_save_button: move |_e: MouseEvent| {
                        ctrl.clicked_attribute_modal_save_button();
                    },
                    change_attribute_selected: move |(id, selected): (usize, bool)| {
                        ctrl.change_attribute_selected(id, selected);
                    },
                    total_attributes: ctrl.get_total_attributes(),
                    selected_attributes: (ctrl.selected_attributes)(),
                    write_attribute: ctrl.get_write_attribute(),
                    edit_write_attribute: move |value: String| {
                        ctrl.edit_write_attribute(value);
                    },
                }
            }
            if let Some(_) = ctrl.get_clicked_attribute_index() {
                AttributeSettingModal {
                    translates: SettingModalTranslates {
                        attribute_setting: props.attribute_setting.clone(),
                        cancel: props.cancel.clone(),
                        save: props.save.clone(),
                    },
                    clicked_cancel_button: move |_e: MouseEvent| {
                        ctrl.clicked_attribute_cancel_button();
                    },
                    clicked_save_button: move |_e: MouseEvent| {
                        ctrl.clicked_attribute_save_button();
                    },
                    change_attribute_setting_value: move |index: usize| {
                        ctrl.change_attribute_setting_value(index);
                    },
                    label: ctrl.get_attribute_modal_label(),
                    attributes: ctrl.get_total_attribute_by_modal(),
                }
            }
            div { class: "flex flex-col w-full h-full justify-start items-center",
                div { class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                    div { class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                        {props.temporary_save}
                    }
                }
                div { class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                    div { class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                        div { class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                            {props.attribute_title}
                        }
                        div { class: "text-black font-semibold text-[22px] mb-[40px]",
                            {props.attribute_description}
                        }
                        div { class: "flex flex-row w-full justify-start items-center mb-[50px]",
                            div { class: "flex items-center space-x-4 w-full",
                                div { class: "flex flex-row w-[138px] h-[50px] rounded-lg border-black border-[1px] justify-center items-center text-black font-semibold text-[22px] mr-[30px]",
                                    {ctrl.get_response_count()}
                                }
                                div { class: "relative w-full",
                                    input {
                                        r#type: "range",
                                        class: "w-full h-2 bg-gray-200 rounded-lg appearance-none focus:outline-none focus:ring-2 focus:ring-blue-400 slider-thumb",
                                        min: "0",
                                        max: (steps.len() - 1).to_string(),
                                        value: (ctrl.bar_index)().to_string(),
                                        oninput: move |e| {
                                            if let Ok(ind) = e.value().parse::<usize>() {
                                                if ind < steps.len() {
                                                    ctrl.bar_index.set(ind);
                                                    ctrl.set_response_count(steps[ind].to_string());
                                                }
                                            }
                                        },
                                    }
                                    div { class: "absolute top-6 w-full flex justify-between text-gray-500 text-sm",
                                        {
                                            steps
                                                .iter()
                                                .enumerate()
                                                .map(|(index, &step)| {
                                                    rsx! {
                                                        span { class: "text-center ml-[10px]", key: "{index}", "{step}" }
                                                    }
                                                })
                                        }
                                    }
                                }

                            }
                        }
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "text-black font-semibold text-[22px] mb-[20px]",
                                {props.attribute_select_label}
                            }
                            for attribute in ctrl.get_search_attributes() {
                                Attribute {
                                    label_image: if attribute.name == "지역" || attribute.name == "연봉" { asset!("/public/images/national.png") } else if attribute.name == "성별" { asset!("/public/images/gender.png") } else { asset!("/public/images/age.png") },
                                    onclick: move |_e| {
                                        ctrl.clicked_attribute(attribute.id);
                                    },
                                    label_name: attribute.name.clone(),
                                    label_value: attribute.initial_value.clone(),
                                }
                            }
                            div { class: "flex flex-row w-full justify-center items-center mb-[40px]",
                                div { class: "flex flex-row w-[200px] h-[50px] rounded-[20px] bg-[#d6d6d6] justify-center items-center mt-[20px]",
                                    img {
                                        class: "flex flex-col pr-[10px]",
                                        src: asset!("/public/images/add.png"),
                                        alt: "add question",
                                    }
                                    div {
                                        class: "text-[20px] font-medium text-black",
                                        onclick: move |_| {
                                            ctrl.change_clicked_add_attribute(true);
                                        },
                                        {props.add_attribute}
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-end items-end mb-[30px]",
                                Link {
                                    to: Route::SelectResponsePage {
                                        lang: props.lang.clone(),
                                        survey_id: props.survey_id.clone(),
                                    },
                                    div { class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                                        {props.cancel.clone()}
                                    }
                                }
                                div {
                                    class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                                    onclick: move |_| {
                                        let survey_id_copy = props.survey_id.clone();
                                        async move {
                                            ctrl.clicked_save_button(props.lang, survey_id_copy.clone()).await;
                                        }
                                    },
                                    {props.save.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeSettingModal(
    translates: SettingModalTranslates,
    clicked_cancel_button: EventHandler<MouseEvent>,
    clicked_save_button: EventHandler<MouseEvent>,
    change_attribute_setting_value: EventHandler<usize>,
    label: String,
    attributes: Vec<AttributeModel>,
) -> Element {
    rsx! {
        div { class: "fixed flex flex-row w-screen h-screen backdrop-blur-sm justify-center items-center z-50",
            AlertModal {
                children: rsx! {
                    div { class: "flex flex-col w-full h-min justify-start items-start",
                        div {
                            class: "flex flex-row w-full justify-end items-center mb-[15px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            Close {
                                width: "23",
                                height: "23",
                                color: "#f7f7f7",
                                border_color: "#aeaeae",
                                button_color: "#000000",
                            }
                        }
                        div { class: "text text-[22px] font-semibold text-black mb-[10px]",
                            {format!("{} {}", label.clone(), translates.attribute_setting)}
                        }
                        div {
                            class: "mb-[30px]",
                            style: "width: 100%; height: 1px; background-color: #d3d3d3",
                        }
                        div { class: "flex flex-col w-full justify-start items-start px-[25px] py-[20px]",
                            for (index , attribute) in attributes.iter().enumerate() {
                                div { class: "flex flex-row w-full justify-start items-start mb-[20px]",
                                    div { class: "flex flex-row w-min h-min justify-center items-center pr-[20px]",
                                        input {
                                            r#type: "radio",
                                            name: label.clone(),
                                            value: index,
                                            checked: attribute.is_select, // 선택 여부
                                            oninput: move |_| change_attribute_setting_value.call(index), // 선택 시 상태 변경
                                        }
                                    }
                                    div { class: "text-black font-normal text-[20px]", {attribute.name.clone()} }
                                }
                            }
                        }
                        div { style: "width: 100%; height: 1px; background-color: #d3d3d3" }
                    }
                },
                width: Some(460),
                max_width: Some(460),
                tail: rsx! {
                    div { class: "flex flex-row w-full h-min justify-end items-end",
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#424242] text-white font-normal text-[20px] mr-[10px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            {translates.cancel.clone()}
                        }
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#2168c3] text-white font-normal text-[20px]",
                            onclick: move |e| {
                                clicked_save_button.call(e);
                            },
                            {translates.save.clone()}
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn AttributeAddModal(
    translates: ModalTranslates,
    clicked_cancel_button: EventHandler<MouseEvent>,
    clicked_add_button: EventHandler<MouseEvent>,
    clicked_save_button: EventHandler<MouseEvent>,
    change_attribute_selected: EventHandler<(usize, bool)>,
    total_attributes: Vec<SelectAttribute>,
    selected_attributes: Vec<SelectAttribute>,
    write_attribute: String,
    edit_write_attribute: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "fixed flex flex-row w-screen h-screen backdrop-blur-sm justify-center items-center z-50",
            AlertModal {
                children: rsx! {
                    div { class: "flex flex-col w-full h-min justify-start items-start",
                        div {
                            class: "flex flex-row w-full justify-end items-center mb-[30px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            Close {
                                width: "23",
                                height: "23",
                                color: "#f7f7f7",
                                border_color: "#aeaeae",
                                button_color: "#000000",
                            }
                        }
                        div { class: "flex flex-row w-full h-[55px] justify-start items-center bg-white border-[1px] border-[#9f9f9f] rounded-xl px-[25px] mb-[40px]",
                            div { class: "mr-[5px]",
                                Search { width: "21", height: "21", color: "#8a8a8a" }
                            }
                            input {
                                class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                                r#type: "text",
                                placeholder: translates.search_hint,
                                value: write_attribute,
                                oninput: move |event| {
                                    edit_write_attribute.call(event.value());
                                },
                            }
                        }
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "text-black font-semibold text-[22px] mb-[18px]",
                                {translates.search_result.clone()}
                            }
                            div { class: "flex flex-row w-full justify-start items-start",
                                for attribute in total_attributes {
                                    if attribute.is_selected {
                                        div {
                                            class: "flex flex-row w-[105px] h-[50px] justify-center items-center rounded-3xl bg-[#2168c3] mr-[5px]",
                                            onclick: move |_| {
                                                change_attribute_selected.call((attribute.id, false));
                                            },
                                            div { class: "mr-[5px]",
                                                Add {
                                                    width: "21",
                                                    height: "21",
                                                    inner_color: "#2168c3",
                                                    color: "#ffffff",
                                                }
                                            }
                                            div { class: "text-[20px] font-normal text-white", {attribute.name} }
                                        }
                                    } else {
                                        div {
                                            class: "flex flex-row w-[105px] h-[50px] justify-center items-center rounded-3xl bg-white border-[1px] border-[#9f9f9f] mr-[5px]",
                                            onclick: move |_| {
                                                change_attribute_selected.call((attribute.id, true));
                                            },
                                            div { class: "mr-[5px]",
                                                Add {
                                                    width: "21",
                                                    height: "21",
                                                    inner_color: "#ffffff",
                                                    color: "#000000",
                                                }
                                            }
                                            div { class: "text-[20px] font-normal text-black", {attribute.name} }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex flex-row w-full h-[50px] justify-center items-center rounded-lg bg-[#d6d6d6] border-[1px] border-[#c8c8c8] mt-[60px] mb-[70px]",
                            onclick: move |e| {
                                clicked_add_button.call(e);
                                edit_write_attribute.call("".to_string());
                            },
                            div { class: "font-medium text-[20px] text-black", {translates.add_attribute.clone()} }
                        }
                        div { style: "width: 100%; height: 1px; background-color: #d3d3d3" }
                        div { class: "flex flex-col w-full justify-start items-start pt-[25px]",
                            div { class: "text-black font-semibold text-[22px] mb-[18px]",
                                {translates.selected_attribute.clone()}
                            }
                            div { class: "flex flex-row w-full justify-start items-start",
                                for attribute in selected_attributes {
                                    div { class: "flex flex-row w-[80px] h-[50px] justify-center items-center rounded-3xl bg-white border-[1px] border-[#9f9f9f] mr-[5px]",
                                        div { class: "text-[20px] font-normal text-black", {attribute.name} }
                                    }
                                }
                            }
                        }
                    }
                },
                width: Some(460),
                max_width: Some(460),
                tail: rsx! {
                    div { class: "flex flex-row w-full h-min justify-end items-end",
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#424242] text-white font-normal text-[20px] mr-[10px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            {translates.cancel}
                        }
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#2168c3] text-white font-normal text-[20px]",
                            onclick: move |e| {
                                clicked_save_button.call(e);
                            },
                            {translates.save}
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn Attribute(
    label_image: Asset,
    label_name: String,
    label_value: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-start mb-[10px]",
            div { class: "flex flex-row w-full justify-start items-center",
                div { class: "flex flex-row w-[245px] h-[80px] justify-start items-center pl-[20px] rounded-l-lg bg-[#2168c3]",
                    img {
                        class: "pr-[15px]",
                        src: label_image,
                        width: 35,
                        height: 35,
                        alt: label_name.clone(),
                    }
                    div { class: "text-white font-normal text-[20px]", {label_name.clone()} }
                }
                div {
                    class: "flex flex-row w-full h-[80px] justify-between items-center bg-[#f9f9f9] rounded-r-lg pl-[30px] pr-[15px]",
                    onclick: move |e| {
                        onclick.call(e);
                    },
                    div { class: "text-black font-normal text-[20px]", {label_value.clone()} }
                    img {
                        src: asset!("/public/images/right-arrow.png"),
                        width: 20,
                        height: 20,
                        alt: "right arrow",
                    }
                }
            }
        }
    }
}
