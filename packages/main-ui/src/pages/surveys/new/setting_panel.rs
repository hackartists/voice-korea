use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{Clear, Remove},
    pages::surveys::new::{
        controller::{Controller, CurrentStep, SelectedPanel},
        i18n::SettingPanelTranslate,
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct SettingPanelProps {
    lang: Language,
}

#[component]
pub fn SettingPanel(props: SettingPanelProps) -> Element {
    let mut ctrl: Controller = use_context();
    let translate: SettingPanelTranslate = translate(&props.lang);
    let selected_panels = ctrl.selected_panels();
    let panels = ctrl.total_panels();
    let total_members = ctrl.get_total_panel_members();

    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "font-medium text-black text-[16px] leading-[22.5px]",
                    "{translate.composition_panel}"
                }
                button {
                    class: "bg-[#2a60d3] rounded-4px px-[14px] py-[8px] font-semibold text-white text-[16px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.open_create_panel_modal().await;
                    },
                    "{translate.create_panel}"
                }
            }

            div {
                class: "flex flex-col w-full justify-start items-start px-[40px] py-[24px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[5px]",
                        "{translate.total_panel_setting}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-[14px]",
                        "{translate.total_panel_setting_description}"
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                        div { class: "flex flex-row w-[200px] font-medium text-[15px] text-black",
                            "{translate.total_panel}"
                        }
                        div { class: "flex flex-row gap-[10px] items-center",
                            div { class: "flex flex-row w-[213px] h-[55px] p-[15px] justify-end items-center font-medium text-[#222222] text-[15px] bg-[#f7f7f7] rounded-[4px]",
                                "{total_members}"
                            }
                            div { class: "font-normal text-black text-[15px]", "{translate.person}" }
                        }
                    }

                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-row w-[200px] font-medium text-[15px] text-black",
                            "{translate.select_panel}"
                        }
                        div { class: "relative w-full",
                            button {
                                class: "flex flex-row w-full min-h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px]",
                                onclick: move |_| {
                                    is_open.set(true);
                                },
                                if selected_panels.len() != 0 {
                                    div { class: "flex flex-wrap flex-1 justify-start items-center gap-[5px]",
                                        for (i , panel) in selected_panels.iter().enumerate() {
                                            PanelLabel {
                                                label: panel.name.clone(),
                                                onclose: move |e: Event<MouseData>| {
                                                    e.stop_propagation();
                                                    e.prevent_default();
                                                    ctrl.remove_selected_panel(i);
                                                },
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            e.prevent_default();
                                            ctrl.remove_all_selected_panel();
                                        },
                                        Remove {
                                            width: "15",
                                            height: "15",
                                            fill: "#555462",
                                        }
                                    }
                                }
                            }
                            if is_open() {
                                div { class: "absolute flex flex-col w-full justify-start items-center shadow-[0px_8px_20px_rgba(20,26,62,0.25)] bg-white py-4 rounded-md",
                                    div { class: "flex flex-row w-full justify-end px-[10px]",
                                        button {
                                            onclick: move |_| {
                                                is_open.set(false);
                                            },
                                            Remove {
                                                width: "15",
                                                height: "15",
                                                fill: "#555462",
                                            }
                                        }
                                    }
                                    for panel in panels.clone() {
                                        if !selected_panels.iter().any(|selected| selected.name == panel.name) {
                                            div {
                                                class: "flex flex-col w-full h-[60px] justify-start items-start py-[9px] bg-white hover:bg-[#f7f7f7] hover:border-l hover:border-l-[#2a60d3] cursor-pointer",
                                                onclick: move |_| {
                                                    ctrl.add_selected_panel(SelectedPanel {
                                                        id: panel.id.clone(),
                                                        name: panel.name.clone(),
                                                        total_count: panel.user_count,
                                                    });
                                                    is_open.set(false);
                                                },
                                                div { class: "flex flex-col w-full px-4",
                                                    div { class: "font-bold text-[15px] text-[#222222] mb-[5px]",
                                                        "{panel.name}"
                                                    }
                                                    div { class: "font-medium text-[10px] text-[#222222]",
                                                        "{translate.total_people}: {panel.user_count}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        for (index , panel) in selected_panels.iter().enumerate() {
                            div { class: "flex flex-row w-full justify-between items-center",
                                div { class: "font-medium text-[#222222] text-[15px]",
                                    "{panel.name}"
                                }
                                div { class: "flex flex-row h-[55px] items-center gap-[10px]",
                                    input {
                                        class: "flex flex-row w-[215px] h-[55px] justify-end items-center rounded-[4px] px-[15px] py-[10px] bg-[#f7f7f7] font-medium text-[#222222] text-[15px]",
                                        r#type: "text",
                                        placeholder: "0",
                                        value: panel.total_count,
                                        onkeydown: move |e: KeyboardEvent| {
                                            let key = e.key();
                                            if key != Key::Backspace && key != Key::Delete {
                                                let s = match key {
                                                    Key::Character(c) => c,
                                                    _ => "".to_string(),
                                                };
                                                if !s.chars().all(|c| c.is_ascii_digit()) {
                                                    e.prevent_default();
                                                }
                                            }
                                        },
                                        oninput: {
                                            let mut ctrl = ctrl.clone();
                                            let prev_count = ctrl.selected_panels()[index].clone().total_count;
                                            move |e: Event<FormData>| {
                                                let maximum_value = ctrl.get_maximum_count(index);
                                                let value = e.value().parse::<u64>().unwrap_or(0);
                                                if maximum_value < value {
                                                    ctrl.change_selected_panel_count(index, maximum_value);
                                                    ctrl.change_total_panel_members(
                                                        total_members - prev_count + maximum_value,
                                                    );
                                                } else {
                                                    ctrl.change_selected_panel_count(index, value);
                                                    ctrl.change_total_panel_members(total_members - prev_count + value);
                                                }
                                            }
                                        },
                                    }

                                    div { class: "font-normal text-black text-[15px]",
                                        "{translate.person}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[20px] text-white mt-[40px]",
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::CreateSurvey);
                    },
                    "{translate.btn_cancel}"
                }
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.save_survey().await;
                    },
                    "{translate.btn_temp_save}"
                }

                button {
                    class: "px-[20px] py-[10px] bg-[#2A60D3] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        ctrl.save_survey().await;
                    },
                    "{translate.btn_complete}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String, onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[5px] gap-[10px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: Event<MouseData>| {
                    onclose.call(e);
                },
                Clear { width: "20", height: "20" }
            }
        }
    }
}
