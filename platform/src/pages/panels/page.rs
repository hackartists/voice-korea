use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{Attribute, Panel};

use crate::{
    components::icons::{ArrowLeft, ArrowRight, RowOption, Search, Switch},
    pages::panels::{
        controller::Controller,
        i18n::{
            AttributeListTranslate, PanelListTranslate, PanelTranslate,
            RemoveAttributeModalTranslate, RemovePanelModalTranslate,
            UpdateAttributeNameModalTranslate, UpdatePanelNameModalTranslate,
        },
    },
    service::popup_service::PopupService,
};

#[derive(Props, Clone, PartialEq)]
pub struct PanelProps {
    lang: Language,
}

#[component]
pub fn PanelPage(props: PanelProps) -> Element {
    let popup_service: PopupService = use_context();
    let ctrl = Controller::new(props.lang, popup_service);
    let panels = ctrl.get_panels();
    let attributes = ctrl.get_attributes();

    let translate: PanelTranslate = translate(&props.lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                    "{translate.panel_title}"
                }
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translate.panel_title}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translate.panel_description}"
            }
            PanelList {
                lang: props.lang,
                panels,
                attributes: attributes.clone(),
                onupdate: move |index: usize| {
                    ctrl.open_update_panel_name(props.lang, index);
                },
                onremove: move |index: usize| {
                    ctrl.open_remove_panel(props.lang, index);
                },
            }
            AttributeList {
                lang: props.lang,
                attributes,
                onupdate: move |index: usize| {
                    ctrl.open_update_attribute_name(props.lang, index);
                },
                onremove: move |index: usize| {
                    ctrl.open_remove_attribute(props.lang, index);
                },
            }
        }
    }
}

#[component]
pub fn AttributeList(
    lang: Language,
    attributes: Vec<Attribute>,
    onupdate: EventHandler<usize>,
    onremove: EventHandler<usize>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut attribute_name = use_signal(|| "".to_string());

    let translate: AttributeListTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.attribute_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: translate.search_hint,
                            value: (attribute_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                attribute_name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        div { class: "w-[25px] h-[25px]",
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute_name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                            button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                "+"
                            }
                        }
                    }
                    for (ind , attribute) in attributes.clone().iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{attribute.name}"
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    for attr in attribute.attribute.clone() {
                                        PanelLabel { label: attr.name }
                                    }
                                }
                                div { class: "group relative",
                                    div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                        button {
                                            RowOption { width: "24", height: "24" }
                                        }
                                        nav {
                                            tabindex: "0",
                                            class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                            ul { class: "py-1",
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onupdate.call(ind);
                                                    },
                                                    "{translate.update_attribute_name}"
                                                }
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onremove.call(ind);
                                                    },
                                                    "{translate.remove_attribute}"
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
        }
    }
}

#[component]
pub fn PanelList(
    lang: Language,
    panels: Vec<Panel>,
    attributes: Vec<Attribute>,
    onupdate: EventHandler<usize>,
    onremove: EventHandler<usize>,
) -> Element {
    let mut ctrl: Controller = use_context();
    let mut is_focused = use_signal(|| false);
    let mut panel_name = use_signal(|| "".to_string());
    let translate: PanelListTranslate = translate(&lang);

    tracing::debug!("panel list: {:?}", panels);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.panel_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: translate.search_hint,
                            value: (panel_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                panel_name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        button {
                            class: "w-[25px] h-[25px]",
                            onclick: move |_| async move {
                                let _ = ctrl.next_panel_clicked().await;
                            },
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.panel_name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.personnel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        for attribute in attributes {
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#555462] font-semibold text-[14px]",
                                    "{attribute.name}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                            button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                "+"
                            }
                        }
                    }
                    for (index , panel) in panels.iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{panel.name}"
                                    }
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{panel.count}"
                                    }
                                }
                                // TODO: Response type matching
                                // for attribute in panel.attribute.clone() {
                                //     div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[5px]",
                                //         for attr in attribute.attribute.clone() {
                                //             PanelLabel { label: attr.name }
                                //         }
                                //     }
                                // }
                                div { class: "group relative",
                                    div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                        button {
                                            RowOption { width: "24", height: "24" }
                                        }
                                        nav {
                                            tabindex: "0",
                                            class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                            ul { class: "py-1",
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onupdate.call(index);
                                                    },
                                                    "{translate.update_panel_name}"
                                                }
                                                li {
                                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                    onclick: move |_| {
                                                        onremove.call(index);
                                                    },
                                                    "{translate.remove_panel}"
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
        }
    }
}

#[component]
pub fn UpdateAttributeNameModal(
    lang: Language,
    onupdate: EventHandler<String>,
    initial_value: String,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: UpdateAttributeNameModalTranslate = translate(&lang);
    let mut attribute_name = use_signal(|| initial_value);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                "{translate.update_attribute_name_description}"
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    "{translate.attribute_name}"
                }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: translate.attribute_name_hint,
                    value: (attribute_name)(),
                    oninput: move |event| {
                        attribute_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    "{translate.attribute_name_warning}"
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |_| {
                            onupdate.call(attribute_name());
                        },
                        "{translate.update}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn UpdatePanelNameModal(
    lang: Language,
    onupdate: EventHandler<String>,
    initial_value: String,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: UpdatePanelNameModalTranslate = translate(&lang);
    let mut panel_name = use_signal(|| initial_value);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                "{translate.update_panel_name_description}"
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    "{translate.panel_name}"
                }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: translate.panel_name_hint,
                    value: (panel_name)(),
                    oninput: move |event| {
                        panel_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]", "{translate.panel_name_warning}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |_| {
                            onupdate.call(panel_name());
                        },
                        "{translate.update}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn RemoveAttributeModal(
    lang: Language,
    remove_click: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemoveAttributeModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.remove_attribute_modal_title}" }
                div { "{translate.remove_attribute_modal_description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            remove_click.call(e);
                        },
                        "{translate.remove}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn RemovePanelModal(
    lang: Language,
    remove_click: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemovePanelModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.remove_panel_modal_title}" }
                div { "{translate.remove_panel_modal_description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            remove_click.call(e);
                        },
                        "{translate.remove}"
                    }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[100px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
