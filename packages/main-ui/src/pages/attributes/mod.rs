#![allow(non_snake_case)]
use crate::components::{
    alert::AlertModal,
    icons::{Add, Close, Search},
};
use controller::SelectAttribute;
use dioxus::prelude::*;
pub mod controller;
pub mod i18n;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::AttributeTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct AttributeProps {
    lang: Language,
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
pub struct TabelTranslates {
    search_result: String,
    tabel_label: Vec<String>,
}

#[component]
pub fn AttributePage(props: AttributeProps) -> Element {
    let mut ctrl = controller::Controller::init(props.lang);
    let translates: AttributeTranslate = translate(&props.lang.clone());
    rsx! {
        div { class: "flex flex-col",
            if (ctrl.enable_modal)() {
                AttributeAddModal {
                    translates: ModalTranslates {
                        search_hint: translates.search_hint.to_string(),
                        search_result: translates.search_result.to_string(),
                        add_attribute: translates.add_attribute.to_string(),
                        selected_attribute: translates.selected_attribute.to_string(),
                        cancel: translates.cancel.to_string(),
                        save: translates.save.to_string(),
                    },
                    clicked_cancel_button: move |_e: MouseEvent| {
                        ctrl.clicked_cancel_button();
                    },
                    clicked_add_button: move |_e: MouseEvent| {
                        ctrl.clicked_add_button();
                    },
                    clicked_save_button: move |_e: MouseEvent| {
                        ctrl.clicked_save_button();
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
            div { class: "flex flex-col w-full h-full justify-start items-center",
                div { class: "flex flex-col max-w-[1230px] min-w-[600px] w-full h-full justify-start items-start mt-[25px] px-[30px]",
                    div { class: "flex flex-col w-full justify-start items-start max-w-[1200px] h-min rounded-[10px] border-solid border border-[#9f9f9f] bg-white px-[33px] py-[55px]",
                        div { class: "text-[#2168c3] font-semibold text-[30px] mb-[45px]",
                            {translates.attribute_status}
                        }
                        div { class: "flex flex-col w-full justify-start items-start mb-[60px]",
                            div { class: "text-[black] font-semibold text-[22px] mb-[20px]",
                                {translates.selected_attribute}
                            }
                            div { class: "flex flex-row w-full justify-start items-start",
                                div { class: "flex flex-row w-[200px] h-[50px] justify-center items-center rounded-3xl bg-[#d6d6d6] border-[1px] border-[#c8c8c8]",
                                    div { class: "pr-[5px] ",
                                        Add {
                                            width: "22",
                                            height: "22",
                                            inner_color: "#ffffff",
                                            color: "#000000",
                                        }
                                    }
                                    div {
                                        class: "text-[20px] font-medium text-black",
                                        onclick: move |_| {
                                            ctrl.show_modal();
                                        },
                                        {translates.add_attribute}
                                    }
                                }
                            }
                        }
                        AttributeTable {
                            translates: TabelTranslates {
                                search_result: translates.search_result.to_string(),
                                tabel_label: vec![
                                    translates.tabel_label_0.to_string(),
                                    translates.tabel_label_1.to_string(),
                                    translates.tabel_label_2.to_string(),
                                ],
                            },
                            search_attributes: (ctrl.search_attributes)(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeTable(translates: TabelTranslates, search_attributes: Vec<SelectAttribute>) -> Element {
    let mut ctrl = controller::use_controller();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[black] font-semibold text-[22px] mb-[35px]",
                {translates.search_result}
            }
            div { class: "flex flex-col w-full h-min justify-start items-start mb-[10px]",
                div { class: "flex flex-row w-full justify-start items-start",
                    div { class: "w-full min-w-[50px] max-w-[100px] h-[25px] pl-[17px]",
                        div { class: "text-[black] font-semibold text-[18px]",
                            {translates.tabel_label[0].clone()}
                        }
                    }
                    div { class: "w-full min-w-[50px] max-w-[150px] h-[25px] pl-[15px]",
                        div { class: "text-[black] font-semibold text-[18px]",
                            {translates.tabel_label[1].clone()}
                        }
                    }
                    div { class: "text-[black] font-semibold text-[18px]",
                        {translates.tabel_label[2].clone()}
                    }
                }
            }
            for (i , attribute) in search_attributes.iter().enumerate() {
                div { class: "flex flex-row w-full h-[55px] justify-start items-center rounded-xl bg-white border-[1px] border-[#9f9f9f] mb-[5px]",
                    div { class: "w-full min-w-[50px] max-w-[100px] h-[25px] pl-[22px]",
                        div { class: "text-[black] font-semibold text-[20px]", {i.to_string()} }
                    }
                    div { class: "w-full min-w-[50px] max-w-[150px] h-[25px] pl-[15px]",
                        div { class: "text-[black] font-semibold text-[20px]",
                            {attribute.name.clone()}
                        }
                    }
                    div { class: "text-[#5e5e5e] font-medium text-[20px]",
                        {ctrl.attribute_value_string(attribute.value.clone())}
                    }
                }
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
