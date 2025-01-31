#![allow(non_snake_case)]
use crate::{
    components::checkbox::Checkbox,
    pages::id::select_response::response_type::controller::{self, Panel, PanelGroup},
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct SelectPanelProps {
    lang: Language,
    select_type: String,
    panel_groups: Vec<PanelGroup>,
    panels: Vec<Panel>,
    select_panel_groups: Vec<u64>,
    select_panels: Vec<u64>,
    total_select_count: u64,

    temporary_save: String,
    select_responsive_panel: String,
    select_response_panel_description: String,
    selection_panel_groups: String,
    cancel: String,
    save: String,
    select_all: String,
    search_results: String,
    panel: String,
}

#[component]
pub fn SelectPanelPage(props: SelectPanelProps) -> Element {
    let mut ctrl = controller::use_controller();

    tracing::debug!("select type: {}", props.select_type);
    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-center",
            div { class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                div { class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                    {props.temporary_save}
                }
            }
            div { class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div { class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                    div { class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        {props.select_responsive_panel}
                    }
                    div { class: "text-black font-semibold text-[22px] mb-[40px]",
                        {props.select_response_panel_description}
                    }
                    if props.select_type == "attribute" {
                        div {
                            AttributeTable {
                                panel_groups: props.panel_groups,
                                select_panel_groups: props.select_panel_groups,
                                total_select_count: props.total_select_count,
                                selection_panel_groups: props.selection_panel_groups,
                            }
                        }
                    } else {
                        div {
                            PanelTable {
                                panels: props.panels,
                                select_panels: props.select_panels,
                                total_select_count: props.total_select_count,

                                select_all: props.select_all,
                                search_results: props.search_results,
                                panel: props.panel,
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-end items-end mb-[30px] mt-[30px]",
                        div {
                            class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                            onclick: move |_| {
                                ctrl.change_step(controller::Step::Attribute);
                            },
                            {props.cancel}
                        }
                        div {
                            class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                            onclick: move |_| {
                                let select_type = props.select_type.clone();
                                async move {
                                    ctrl.clicked_panel_save_button(select_type.clone()).await;
                                }
                            },
                            {props.save}
                        }
                    }
                }
            }


        }
    }
}

#[component]
pub fn PanelRow(
    panel: Panel,
    background_color: String,
    panel_string: String,
    index: usize,
) -> Element {
    let mut ctrl = controller::use_controller();
    rsx! {
        div {
            class: "flex flex-row w-full h-[80px] justify-between items-start p-[30px]",
            style: format!("background-color: {}", background_color),
            div { class: "flex flex-row w-full justify-start items-start",
                div { class: "flex flex-row w-min h-min justify-center items-center pr-[20px]",
                    Checkbox {
                        id: format!("check panel {}", index),
                        onchange: move |_| {
                            let value = ctrl.get_select_panels().contains(&(index as u64));
                            ctrl.change_select_panels(index as u64, !value);
                        },
                        checked: ctrl.get_select_panels().contains(&(index as u64)),
                    }
                }
                div { class: "w-[100px] text-black font-semibold text-[20px] pr-[20px]",
                    {format!("{} {}", panel_string, index + 1)}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.region.clone()}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.gender.clone()}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.age.clone()}
                }
                div { class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.payload.clone()}
                }
            }
        }
    }
}

#[component]
pub fn PanelTable(
    panels: Vec<Panel>,
    select_panels: Vec<u64>,
    total_select_count: u64,

    select_all: String,
    search_results: String,
    panel: String,
) -> Element {
    let mut ctrl = controller::use_controller();
    rsx! {
        div { class: "flex flex-col justify-start items-start w-full",
            div { class: "flex flex-row w-full h-full justify-between items-end mb-[10px]",
                div { class: "flex flex-row",
                    div { class: "text-xl font-normal text-[#5e5e5e] mr-[5px]",
                        {format!("{} : ", search_results)}
                    }
                    //FIXME: fix to real result
                    div { class: "text-xl font-semibold text-[#2168c3]", "250명" }
                }
                div { class: "flex flex-row",
                    div { class: "text-xl font-normal text-[#5e5e5e] mr-[3px]", "현재" }
                    div { class: "text-xl font-semibold text-[#2168c3] mr-[3px]",
                        {format!("{}명", total_select_count)}
                    }
                    div { class: "text-xl font-normal text-[#5e5e5e]", "선택 중" }
                }
            }
            div { class: "flex flex-col w-full h-min justify-start items-start",
                div { class: "flex flex-row justify-start items-start mb-[10px]",
                    div { class: "mr-[6px]",
                        Checkbox {
                            id: format!("check all"),
                            onchange: move |_| {
                                ctrl.clicked_panels_all();
                            },
                            checked: ctrl.get_click_total_check(),
                        }
                    }
                    div { class: "text-[20px] font-normal text-black", {select_all} }
                }
                for (i , d) in panels.iter().enumerate() {
                    if i % 2 == 0 {
                        PanelRow {
                            panel: d.clone(),
                            background_color: "#f9f9f9".to_string(),
                            panel_string: panel.clone(),
                            index: i,
                        }
                    } else {
                        PanelRow {
                            panel: d.clone(),
                            background_color: "#ffffff".to_string(),
                            panel_string: panel.clone(),
                            index: i,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeRow(
    background_color: String,
    selection_panel_groups: String,
    index: usize,
    panel: PanelGroup,
) -> Element {
    let mut ctrl = controller::use_controller();
    rsx! {
        div {
            class: "flex flex-row w-full h-[100px] justify-between items-start p-[30px]",
            style: format!("background-color: {}", background_color),
            div { class: "flex flex-row w-full justify-start items-start",
                div { class: "flex flex-row w-min h-min justify-center items-center pr-[20px]",
                    Checkbox {
                        id: format!("check attribute {}", index),
                        onchange: move |_| {
                            let value = ctrl.get_select_panel_groups().contains(&(index as u64));
                            ctrl.change_select_panel_groups(index as u64, !value);
                        },
                        checked: ctrl.get_select_panel_groups().contains(&(index as u64)),
                    }
                }
                div { class: "w-[190px] text-black font-semibold text-[20px] pr-[20px]",
                    {format!("{} {}", selection_panel_groups, index + 1)}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.region.clone()}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.gender.clone()}
                }
                div { class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.age.clone()}
                }
                div { class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                    {panel.payload.clone()}
                }
            }
            div { class: "flex flex-row w-min justify-start items-start",
                div { class: "text-black font-medium text-[20px] mr-[5px]", "총 " }
                div { class: "text-[#1e5eaf] font-medium text-[20px]", {panel.value.to_string()} }
                div { class: "text-black font-medium text-[20px]", "명" }
            }
        }
    }
}

#[component]
pub fn AttributeTable(
    panel_groups: Vec<PanelGroup>,
    select_panel_groups: Vec<u64>,
    total_select_count: u64,

    selection_panel_groups: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col justify-start items-start w-full",
            div { class: "flex flex-row w-full h-full justify-end items-end mb-[10px]",
                div { class: "text-xl font-normal text-[#5e5e5e] mr-[3px]", "현재" }
                div { class: "text-xl font-semibold text-[#2168c3] mr-[3px]",
                    {format!("{}명", total_select_count)}
                }
                div { class: "text-xl font-normal text-[#5e5e5e]", "선택 중" }
            }
            div { class: "flex flex-col w-full h-min justify-start items-start",
                for (i , d) in panel_groups.iter().enumerate() {
                    if i % 2 == 0 {
                        AttributeRow {
                            background_color: "#f9f9f9".to_string(),
                            selection_panel_groups: selection_panel_groups.clone(),
                            index: i,
                            panel: d.clone(),
                        }
                    } else {
                        AttributeRow {
                            background_color: "#ffffff".to_string(),
                            selection_panel_groups: selection_panel_groups.clone(),
                            index: i,
                            panel: d.clone(),
                        }
                    }
                }
            }
        }
    }
}
