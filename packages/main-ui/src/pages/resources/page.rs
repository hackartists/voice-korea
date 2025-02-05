#![allow(unused)]

use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    AccessLevel, Field, ResourceGetResponse, ResourceSummary, ResourceType, Source, UsagePurpose,
};

use crate::{
    components::icons::{Navigation, RowOption, Search as SearchIcon, Switch, Upload},
    pages::resources::i18n::ResourceTranslate,
};

#[cfg(feature = "web")]
use dioxus::html::HasFileData;

#[cfg(feature = "web")]
use dioxus::web::WebEventExt;

#[cfg(feature = "web")]
use web_sys::window;

use super::controller::{Controller, OrderBy, SortOrder, UpdateResource};

struct DisplayOption<T>(Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "값이 없습니다."),
        }
    }
}
#[component]
pub fn Badge(value: String, #[props(default = "".to_string())] class: String) -> Element {
    rsx! {
        div { class: "inline-block whitespace-nowrap rounded-full bg-black text-white font-semibold {class}",
            div { class: "px-2.5 py-0.5 flex justify-center items-center", {value} }
        }
    }
}

#[component]
pub fn TableRow(
    lang: Language,
    resource: ResourceSummary,
    is_editing: bool,
    onedit: EventHandler<bool>,
    onupdate: EventHandler<UpdateResource>,
    ondownload: EventHandler<String>,
) -> Element {
    let translate: ResourceTranslate = translate(&lang);
    let no_selection_text = translate.no_selection;

    let id = resource.id.parse::<i32>().unwrap();

    let resource_type = match resource.resource_type {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut resource_type_options = ResourceType::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    resource_type_options.insert(0, no_selection_text.to_string());

    let field = match resource.field {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut field_options = Field::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    field_options.insert(0, no_selection_text.to_string());

    let usage_purpose = match resource.usage_purpose {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut usage_purpose_options = UsagePurpose::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    usage_purpose_options.insert(0, no_selection_text.to_string());

    let source = match resource.source {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut source_options = Source::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    source_options.insert(0, no_selection_text.to_string());

    let access_level = match resource.access_level {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut access_level_options = AccessLevel::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    access_level_options.insert(0, no_selection_text.to_string());

    rsx! {
        tr {
            tabindex: "0",
            class: "[&>*]:px-6 [&>*]:py-3 [&>*]:text-center h-[56px] [&>*]:w-full [&>*]:text-sm [&>*]:font-semibold [&>*]:text-[#34333e]",
            style: if is_editing { "background: #F7F7F7;" } else { "" },
            onclick: move |evt: MouseEvent| {
                if !is_editing {
                    onedit.call(true);
                    evt.stop_propagation();
                }
            },

            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: resource_type,
                options: resource_type_options,
                onchange: move |v: String| {
                    let resource_type = ResourceType::from_str(&v).ok();
                    onupdate.call(UpdateResource::ResourceType(resource_type));
                },
            }
            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: field,
                options: field_options,
                onchange: move |v: String| {
                    let field = Field::from_str(&v).ok();
                    onupdate.call(UpdateResource::Field(field));
                },
            }
            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: usage_purpose,

                options: usage_purpose_options,
                onchange: move |v: String| {
                    let purpose = UsagePurpose::from_str(&v).ok();
                    onupdate.call(UpdateResource::UsagePurpose(purpose));
                },
            }
            td { "{resource.title}" }
            //TODO: Use Resource Data
            td {
                Badge {
                    class: "text-white bg-[#2a60d3] rounded-[4px] px-[5px] py-[2px]",
                    value: "공론명",
                }
            }
            EditableTableBodyCell {
                default_value: source,
                edit_mode: is_editing,
                options: source_options,
                onchange: move |v: String| {
                    let source = Source::from_str(&v).ok();
                    onupdate.call(UpdateResource::Source(source));
                },
            }
            EditableTableBodyCell {
                default_value: access_level,
                edit_mode: is_editing,
                options: access_level_options,
                onchange: move |v: String| {
                    let access_level = AccessLevel::from_str(&v).ok();
                    onupdate.call(UpdateResource::AccessLevel(access_level));
                },
            }
            td { class: "font-semibold",
                "{Controller::convert_timestamp_to_date(resource.updated_at)}"
            }
            td {
                onclick: move |event: Event<MouseData>| {
                    event.stop_propagation();
                    event.prevent_default();
                },
                button {
                    class: "text-[#2a60d3] font-semibold text-[14px]",
                    onclick: move |_| {
                        ondownload.call(resource.id.clone());
                    },
                    "{translate.download}"
                }
            }
            td {
                More {
                    options: vec!["OK".to_string(), "None".to_string()],
                    onclick: move |_| {
                        tracing::debug!("Resource More Clicked");
                    },
                }
            }
        }
    }
}

#[component]
pub fn EditableTableBodyCell(
    edit_mode: bool,
    options: Vec<String>,
    default_value: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut value: Signal<String> = use_signal(|| default_value);
    rsx! {
        td {
            onclick: move |evt| {
                if edit_mode {
                    evt.stop_propagation();
                }
            },
            if edit_mode {
                select {
                    class: "focus:outline-none w-full text-inherit bg-transparent",
                    value: value(),
                    onchange: move |evt: FormEvent| {
                        tracing::debug!("updated value : {}", evt.value());
                        value.set(evt.value());
                        onchange.call(evt.value());
                    },
                    for option in options {
                        option {
                            class: "text-center",
                            value: option.clone(),
                            selected: value() == option,
                            div { class: "flex justify-between items-center", "{option}" }
                        }
                    }
                }
            } else {
                div { class: "text-[#222222] text-sm font-semibold text", "{value}" }
            }
        }
    }
}

#[component]
pub fn TableHeaderCell(
    #[props(default = "".to_string())] class: String,
    value: String,
    #[props(default = None)] order: Option<SortOrder>, // None: default, true: asc, false: desc
    onclick: EventHandler<MouseEvent>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let v = order.clone();
    rsx! {
        th {
            class: "py-[18px] text-[#7C8291] font-semibold text-sm {class} cursor-pointer",
            onclick: move |evt| {
                onclick.call(evt);
            },
            div { class: "flex flex-row gap-[10px] justify-center items-center",
                "{value}"
                if !disabled {
                    if order.is_none() {
                        Switch { width: "18px", height: "18px" }
                    } else if order.unwrap() == SortOrder::Asc {
                        Navigation {}
                    } else {
                        div { class: "rotate-180", Navigation {} }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Search(placeholder: String, onsearch: EventHandler<String>) -> Element {
    let mut value = use_signal(String::default);
    rsx! {
        div { class: "text-sm gap-2 flex flex-row flex-[0_1_590px] justify-between items-center rounded-lg pl-[18px] pr-[15px] py-[10px] bg-[#f7f7f7] border border-[#7c8292] focus:bg-[#ffffff] focus:border-[#2a60d3]",
            input {
                class: "flex flex-row bg-transparent focus:outline-none flex-1",
                r#type: "text",
                placeholder,
                value: value(),
                oninput: move |event| {
                    value.set(event.value());
                },
            }
            SearchIcon { width: "18", height: "18", color: "#7c8292" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ResourceProps {
    lang: Language,
}

#[component]
pub fn More(options: Vec<String>, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "group relative",
            div {
                class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                onclick: move |event: Event<MouseData>| {
                    event.stop_propagation();
                    event.prevent_default();
                    onclick.call(event);
                },
                button {
                    RowOption { width: "24", height: "24" }
                }
                nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                    ul { class: "py-1",
                        li {
                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                            onclick: move |_event| {},
                            // "{translate.update_material_li}"
                            "OK"
                        }
                        li {
                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer whitespace-nowrap ",
                            onclick: move |_event| {},
                            "None"
                                                // "{translate.remove_material_li}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ResourcePage(props: ResourceProps) -> Element {
    let translate: ResourceTranslate = translate(&props.lang);
    let mut ctrl = Controller::new(props.lang)?;
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start mb-6",
                div { class: "text-[#b3b3b3] font-medium text-sm mb-2", "{translate.title}" }
                div { class: "text-[#222222] text-[28px] font-semibold leading-[42px] mb-10",
                    "{translate.title}"
                }
            }
            div { class: "text-[#35343f] font-normal text-sm mb-10", "{translate.description}" }
            div { class: "flex flex-col w-full p-5 bg-white rounded-lg",
                div { class: "flex-1 flex justify-between",
                    Search {
                        placeholder: translate.placeholder.to_string(),
                        onsearch: move |p| {
                            tracing::debug!("Params: {:?}", p);
                        },
                    }
                    button {
                        onclick: move |_| {
                            ctrl.open_create_resource_modal();
                        },
                        div { class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                            Upload { width: "24", height: "24" }
                            div { class: "text-white font-semibold text-[#16px]",
                                "{translate.upload_resource}"
                            }
                        }
                    }
                }
                div { class: "overflow-x-scroll my-[30px] border border-[#bfc8d9] rounded",
                    table { class: "border-collapse w-full table-fixed",
                        colgroup {
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[120px]" }
                            col { class: "min-w-[180px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[90px] max-w-7xl" }
                        }
                        thead {
                            tr {
                                TableHeaderCell {
                                    value: translate.metadata_type,
                                    order: ctrl.is_sorted_by(OrderBy::ResourceType),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::ResourceType);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.field,
                                    order: ctrl.is_sorted_by(OrderBy::Field),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::Field);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.purpose,
                                    order: ctrl.is_sorted_by(OrderBy::UsagePurpose),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::UsagePurpose);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.header_title,
                                    order: ctrl.is_sorted_by(OrderBy::Title),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::Title);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.linked_deliberation_survey,
                                    order: ctrl.is_sorted_by(OrderBy::LinkedDeliberationSurvey),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::LinkedDeliberationSurvey);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.source,
                                    order: ctrl.is_sorted_by(OrderBy::Source),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::Source);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.authority,
                                    order: ctrl.is_sorted_by(OrderBy::AccessLevel),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::AccessLevel);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.last_modified_date,
                                    order: ctrl.is_sorted_by(OrderBy::LastModifiedDate),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::LastModifiedDate);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.function,
                                    disabled: true,
                                    onclick: move |v| {},
                                }
                                th {
                                }
                            }
                        }
                        tbody {
                            for (index , resource) in ctrl.get_resources().into_iter().enumerate() {
                                TableRow {
                                    key: format!("resource-{}", index),
                                    lang: props.lang,
                                    is_editing: ctrl.is_editing(index as i32),
                                    resource: resource.clone(),
                                    onedit: move |v: bool| {
                                        if !v {
                                            ctrl.handle_change_editing_row(-1);
                                        } else {
                                            ctrl.handle_change_editing_row(index as i32);
                                        }
                                    },
                                    ondownload: move |id| {
                                        tracing::debug!("Download Button Clicked: {}", id);
                                    },
                                    onupdate: move |update_field| {
                                        ctrl.handle_update_resource(index, update_field);
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start",
//             div { class: "flex flex-col w-full justify-start items-start",
//                 div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
//                     "{translate.resource_title}"
//                 }
//             }
//             div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
//                 "{translate.resource_title}"
//             }
//             div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
//                 "{translate.resource_description}"
//             }

//             div { class: "flex flex-col w-full justify-start items-start mb-[50px]",
//                 div {
//                     class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
//                     style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
//                     div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
//                         div {
//                             class: format!(
//                                 "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
//                                 if (is_focused)() {
//                                     "bg-[#ffffff] border border-[#2a60d3]"
//                                 } else {
//                                     "bg-[#f7f7f7] border border-[#7c8292]"
//                                 },
//                             ),
//                             input {
//                                 class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
//                                 r#type: "text",
//                                 placeholder: translate.search_hint,
//                                 value: (resource_name)(),
//                                 onfocus: move |_| {
//                                     is_focused.set(true);
//                                 },
//                                 onblur: move |_| {
//                                     is_focused.set(false);
//                                 },
//                                 oninput: move |event| {
//                                     resource_name.set(event.value());
//                                 },
//                             }
//                             SearchIcon { width: "18", height: "18", color: "#7c8292" }
//                         }

//                         button {
//                             onclick: move |_| {
//                                 ctrl.open_create_material(props.lang);
//                             },
//                             div { class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
//                                 Upload { width: "24", height: "24" }
//                                 div { class: "text-white font-semibold text-[#16px]",
//                                     "{translate.upload_material}"
//                                 }
//                             }
//                         }
//                     }
//                     div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
//                         div { class: "flex flex-row w-full h-[55px] justify-start items-center",
//                             div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.metadata_type}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.field}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[180px] min-w-[180px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.purpose}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.title}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.linked_surveys}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.source}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.authority}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.last_modified_date}"
//                                 }
//                                 Switch { width: "19", height: "19" }
//                             }
//                             div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
//                                 div { class: "text-[#555462] font-semibold text-[14px]",
//                                     "{translate.function}"
//                                 }
//                             }
//                             div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
//                         }

//                         for (index , resource) in resources.clone().iter().enumerate() {
//                             div { class: "flex flex-col w-full justify-start items-start",
//                                 div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
//                                 div {
//                                     class: "flex flex-row w-full h-[55px]",
//                                     onclick: move |_| {
//                                         clicked_resources.set(index);
//                                     },
//                                     div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
//                                         if clicked_resources() == index {
//                                             select {
//                                                 class: "bg-transparent focus:outline-none",
//                                                 onchange: {
//                                                     let resource = resource.clone();
//                                                     let ctrl = ctrl.clone();
//                                                     let mut req: UpdateMetadataRequest = resource.into();
//                                                     move |evt: FormEvent| {
//                                                         req.metadata_type = ctrl.metadata_type_from_str(evt.value());
//                                                         let req = req.clone();
//                                                         async move {
//                                                             let _ = ctrl.update_metadata(index, req.clone()).await;
//                                                         }
//                                                     }
//                                                 },
//                                                 option {
//                                                     value: "",
//                                                     disabled: true,
//                                                     selected: resource.metadata_type.is_none(),
//                                                     "{translate.select_type}"
//                                                 }
//                                                 for res in total_types.clone() {
//                                                     option {
//                                                         value: res.clone(),
//                                                         selected: res.clone()
//                                                             == ctrl
//                                                                 .translate_metadata_type(
//                                                                     props.lang,
//                                                                     resource.metadata_type.clone().unwrap(),
//                                                                 ),
//                                                         "{res}"
//                                                     }
//                                                 }
//                                             }
//                                         } else {
//                                             div { class: "text-[#555462] font-semibold text-[14px]",
//                                                 if resource.metadata_type.is_none() {
//                                                     "{translate.not_exists}"
//                                                 } else {
//                                                     {
//                                                         ctrl.translate_metadata_type(props.lang, resource.metadata_type.clone().unwrap())
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
//                                         if clicked_resources() == index {
//                                             select {
//                                                 class: "bg-transparent focus:outline-none",
//                                                 onchange: {
//                                                     let resource = resource.clone();
//                                                     let ctrl = ctrl.clone();
//                                                     let mut req: UpdateMetadataRequest = resource.into();
//                                                     move |evt: FormEvent| {
//                                                         req.metadata_field = ctrl.metadata_field_from_str(evt.value());
//                                                         let req = req.clone();
//                                                         async move {
//                                                             let _ = ctrl.update_metadata(index, req.clone()).await;
//                                                         }
//                                                     }
//                                                 },
//                                                 option {
//                                                     value: "",
//                                                     disabled: true,
//                                                     selected: resource.metadata_field.is_none(),
//                                                     "{translate.select_field}"
//                                                 }
//                                                 for field in total_fields.clone() {
//                                                     option {
//                                                         value: field.clone(),
//                                                         selected: field.clone()
//                                                             == ctrl
//                                                                 .translate_metadata_field(
//                                                                     props.lang,
//                                                                     resource.metadata_field.clone().unwrap(),
//                                                                 ),
//                                                         "{field}"
//                                                     }
//                                                 }
//                                             }
//                                         } else {
//                                             div { class: "text-[#555462] font-semibold text-[14px]",
//                                                 if resource.metadata_field.is_none() {
//                                                     "{translate.not_exists}"
//                                                 } else {
//                                                     {
//                                                         ctrl.translate_metadata_field(
//                                                             props.lang,
//                                                             resource.metadata_field.clone().unwrap(),
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     div { class: "flex flex-row w-[180px] min-w-[180px] h-full justify-center items-center",
//                                         if clicked_resources() == index {
//                                             select {
//                                                 class: "bg-transparent focus:outline-none",
//                                                 onchange: {
//                                                     let resource = resource.clone();
//                                                     let ctrl = ctrl.clone();
//                                                     let mut req: UpdateMetadataRequest = resource.into();
//                                                     move |evt: FormEvent| {
//                                                         req.metadata_purpose = ctrl.metadata_purpose_from_str(evt.value());
//                                                         let req = req.clone();
//                                                         async move {
//                                                             let _ = ctrl.update_metadata(index, req.clone()).await;
//                                                         }
//                                                     }
//                                                 },
//                                                 option {
//                                                     value: "",
//                                                     disabled: true,
//                                                     selected: resource.metadata_purpose.is_none(),
//                                                     "{translate.select_purpose}"
//                                                 }
//                                                 for purpose in total_purposes.clone() {
//                                                     option {
//                                                         value: purpose.clone(),
//                                                         selected: purpose.clone()
//                                                             == ctrl
//                                                                 .translate_metadata_purpose(
//                                                                     props.lang,
//                                                                     resource.metadata_purpose.clone().unwrap(),
//                                                                 ),
//                                                         "{purpose}"
//                                                     }
//                                                 }
//                                             }
//                                         } else {
//                                             div { class: "text-[#555462] font-semibold text-[14px]",
//                                                 if resource.metadata_purpose.is_none() {
//                                                     "{translate.not_exists}"
//                                                 } else {
//                                                     {
//                                                         ctrl.translate_metadata_purpose(
//                                                             props.lang,
//                                                             resource.metadata_purpose.clone().unwrap(),
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     div { class: "flex flex-row flex-1 h-full justify-center items-center",
//                                         div { class: "text-[#555462] font-semibold text-[14px]",
//                                             "{resource.name.clone()}"
//                                         }
//                                     }
//                                     div { class: "flex flex-row flex-1 h-full justify-center items-center" }
//                                     div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
//                                         if clicked_resources() == index {
//                                             select {
//                                                 class: "bg-transparent focus:outline-none",
//                                                 onchange: {
//                                                     let resource = resource.clone();
//                                                     let ctrl = ctrl.clone();
//                                                     let mut req: UpdateMetadataRequest = resource.into();
//                                                     move |evt: FormEvent| {
//                                                         req.metadata_source = ctrl.metadata_source_from_str(evt.value());
//                                                         let req = req.clone();
//                                                         async move {
//                                                             let _ = ctrl.update_metadata(index, req.clone()).await;
//                                                         }
//                                                     }
//                                                 },
//                                                 option {
//                                                     value: "",
//                                                     disabled: true,
//                                                     selected: resource.metadata_source.is_none(),
//                                                     "{translate.select_source}"
//                                                 }
//                                                 for source in total_resources.clone() {
//                                                     option {
//                                                         value: source.clone(),
//                                                         selected: source.clone()
//                                                             == ctrl
//                                                                 .translate_metadata_source(
//                                                                     props.lang,
//                                                                     resource.metadata_source.clone().unwrap(),
//                                                                 ),
//                                                         "{source}"
//                                                     }
//                                                 }
//                                             }
//                                         } else {
//                                             div { class: "text-[#555462] font-semibold text-[14px]",
//                                                 if resource.metadata_source.is_none() {
//                                                     "{translate.not_exists}"
//                                                 } else {
//                                                     {
//                                                         ctrl.translate_metadata_source(
//                                                             props.lang,
//                                                             resource.metadata_source.clone().unwrap(),
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
//                                         if clicked_resources() == index {
//                                             select {
//                                                 class: "bg-transparent focus:outline-none",
//                                                 onchange: {
//                                                     let resource = resource.clone();
//                                                     let ctrl = ctrl.clone();
//                                                     let mut req: UpdateMetadataRequest = resource.into();
//                                                     move |evt: FormEvent| {
//                                                         req.metadata_authority = ctrl.metadata_authority_from_str(evt.value());
//                                                         let req = req.clone();
//                                                         async move {
//                                                             let _ = ctrl.update_metadata(index, req.clone()).await;
//                                                         }
//                                                     }
//                                                 },
//                                                 option {
//                                                     value: "",
//                                                     disabled: true,
//                                                     selected: resource.metadata_authority.is_none(),
//                                                     "{translate.select_authority}"
//                                                 }
//                                                 for authority in total_authorities.clone() {
//                                                     option {
//                                                         value: authority.clone(),
//                                                         selected: authority.clone()
//                                                             == ctrl
//                                                                 .translate_metadata_authority(
//                                                                     props.lang,
//                                                                     resource.metadata_authority.clone().unwrap(),
//                                                                 ),
//                                                         "{authority}"
//                                                     }
//                                                 }
//                                             }
//                                         } else {
//                                             div { class: "text-[#555462] font-semibold text-[14px]",
//                                                 if resource.metadata_authority.is_none() {
//                                                     "{translate.not_exists}"
//                                                 } else {
//                                                     {
//                                                         ctrl.translate_metadata_authority(
//                                                             props.lang,
//                                                             resource.metadata_authority.clone().unwrap(),
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
//                                         div { class: "text-[#555462] font-semibold text-[14px]",
//                                             {ctrl.convert_timestamp_to_date(resource.updated_at)}
//                                         }
//                                     }
//                                     div {
//                                         class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
//                                         onclick: move |event: Event<MouseData>| {
//                                             event.stop_propagation();
//                                             event.prevent_default();
//                                         },
//                                         button {
//                                             class: "text-[#2a60d3] font-semibold text-[14px]",
//                                             onclick: {
//                                                 let resource = resource.clone();
//                                                 move |_| {
//                                                     #[cfg(feature = "web")]
//                                                     {
//                                                         for link in resource.urls.clone() {
//                                                             if let Some(win) = window() {
//                                                                 win.open_with_url_and_target(&link, "_blank").ok();
//                                                             }
//                                                         }
//                                                     }
//                                                     #[cfg(not(feature = "web"))]
//                                                     {
//                                                         let _ = &resource;
//                                                     }
//                                                 }
//                                             },
//                                             "{translate.download}"
//                                         }
//                                     }
//                                     div { class: "group relative",
//                                         div {
//                                             class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
//                                             onclick: move |event: Event<MouseData>| {
//                                                 event.stop_propagation();
//                                                 event.prevent_default();
//                                             },
//                                             button {
//                                                 RowOption { width: "24", height: "24" }
//                                             }
//                                             nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
//                                                 ul { class: "py-1",
//                                                     li {
//                                                         class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
//                                                         onclick: move |_event| {
//                                                             ctrl.open_update_material(props.lang, index);
//                                                         },
//                                                         "{translate.update_material_li}"
//                                                     }
//                                                     li {
//                                                         class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
//                                                         onclick: move |_event| {
//                                                             ctrl.open_remove_material(props.lang, index);
//                                                         },
//                                                         "{translate.remove_material_li}"
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }

//                     //pagenation
//                     div { class: "flex flex-row w-full justify-center items-center mt-[20px]",
//                         div { class: "mr-[20px] w-[24px] h-[24px]",
//                             ArrowLeft { width: "24", height: "24" }
//                         }
//                         //FIXME: add pagination by variable(page, index)
//                         for i in 0..10 {
//                             if i == 0 {
//                                 div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
//                                     "{i + 1}"
//                                 }
//                             } else {
//                                 div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
//                                     "{i + 1}"
//                                 }
//                             }
//                         }
//                         div { class: "flex flex-row ml-[12px] w-[60px] h-[40px] justify-center items-center font-bold text-[15px] text-[#0d1732]",
//                             "More"
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// pub struct File {
//     pub file_type: String,
//     pub file_name: String,
//     pub url: String,
//     pub file_size: String,
//     pub bytes: Option<Vec<u8>>,
// }

// #[component]
// pub fn UpdateMaterialModal(
//     lang: Language,
//     initial_title: String,
//     onupload: EventHandler<Vec<File>>,
//     onclose: EventHandler<MouseEvent>,
// ) -> Element {
//     let translate: UploadMaterialModalTranslate = translate(&lang);
//     let mut title: Signal<String> = use_signal(|| initial_title);
//     let mut files: Signal<Vec<File>> = use_signal(|| vec![]);

//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start",
//             div { class: "text-[#6d6d6d] font-normal text-[14px] mb-[40px]",
//                 "{translate.upload_material_modal_description}"
//             }

//             div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
//                 div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
//                     "{translate.input_title}"
//                 }
//                 div {
//                     class: format!(
//                         "flex flex-row w-full h-[45px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mb-[5px]",
//                     ),
//                     input {
//                         class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
//                         r#type: "text",
//                         value: title(),
//                         placeholder: translate.input_hint,
//                         oninput: move |e| {
//                             title.set(e.value());
//                         },
//                     }
//                 }
//                 div { class: "font-normal text-[#222222] text-[13px]", "{translate.input_info}" }
//             }

//             div { class: "flex flex-col w-full justify-start items-start",
//                 div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
//                     "{translate.classification}"
//                 }
//                 DirectUpload {
//                     lang,
//                     onchange: move |files| {
//                         tracing::debug!("files: {:?}", files);
//                     },
//                 }
//             }

//             div { class: "flex flex-row w-full justify-start items-start font-normal text-[#6d6d6d] text-[14px] mt-[40px] mb-[20px]",
//                 "총 5개 자료 업로드"
//             }
//             div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
//                 button {
//                     class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
//                     onclick: move |_| async move {
//                         onupload.call(files());
//                     },
//                     Edit { width: "24", height: "24" }
//                     div { class: "text-white font-semibold text-[#16px]", "{translate.update}" }
//                 }
//                 button {
//                     class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
//                     onclick: move |e: Event<MouseData>| {
//                         onclose.call(e);
//                     },
//                     "{translate.cancel}"
//                 }
//             }
//         }
//     }
// }

// #[component]
// pub fn CreateMaterialModal(
//     lang: Language,
//     onupload: EventHandler<CreateMetadataRequest>,
//     onclose: EventHandler<MouseEvent>,

//     total_types: Vec<String>,
//     total_fields: Vec<String>,
//     total_purposes: Vec<String>,
//     total_resources: Vec<String>,
//     total_authorities: Vec<String>,
// ) -> Element {
//     let translate: CreateMaterialModalTranslate = translate(&lang);

//     let mut selected_type = use_signal(|| translate.no_selection.to_string());
//     let mut selected_field = use_signal(|| translate.no_selection.to_string());
//     let mut selected_purpose = use_signal(|| translate.no_selection.to_string());
//     let mut selected_source = use_signal(|| translate.no_selection.to_string());
//     let mut selected_authority = use_signal(|| translate.no_selection.to_string());

//     let metadata_urls: Signal<Vec<String>> = use_signal(|| vec![]);
//     let mut name: Signal<String> = use_signal(|| "".to_string());

//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start",
//             div { class: "text-[#6d6d6d] font-normal text-[14px] mb-[40px]",
//                 "{translate.create_material_modal_translate}"
//             }
//             div { class: "flex flex-row w-full justify-start items-start gap-[40px]",
//                 div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start mr-[40px]",
//                     div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
//                         div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
//                             "{translate.input_title}"
//                         }
//                         div {
//                             class: format!(
//                                 "flex flex-row w-full h-[45px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mb-[5px]",
//                             ),
//                             input {
//                                 class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
//                                 r#type: "text",
//                                 value: name(),
//                                 placeholder: translate.input_hint,
//                                 oninput: move |e| {
//                                     name.set(e.value());
//                                 },
//                             }
//                         }
//                         div { class: "font-normal text-[#222222] text-[13px]", "{translate.input_info}" }
//                     }

//                     div { class: "flex flex-col w-full justify-start items-start",
//                         div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
//                             "{translate.classification}"
//                         }
//                         DirectUpload {
//                             lang,
//                             onchange: move |v| {
//                                 tracing::debug!("datas: {:?}", v);
//                             },
//                         }
//                     }
//                 }

//                 div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start gap-[40px]",
//                     div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
//                         div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
//                             "{translate.classification}"
//                         }

//                         div { class: "flex flex-col w-full justify-start items-start p-[24px] bg-white border border-[#bfc8d9] rounded-[8px]",
//                             div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
//                                 div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
//                                     "{translate.metadata_type}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
//                                     select {
//                                         class: "bg-transparent focus:outline-none w-full",
//                                         value: selected_type(),
//                                         onchange: move |e| {
//                                             selected_type.set(e.value());
//                                         },
//                                         option {
//                                             value: "",
//                                             disabled: true,
//                                             selected: selected_type() == translate.no_selection.to_string(),
//                                             "{translate.no_selection}"
//                                         }
//                                         for metadata_type in total_types.clone() {
//                                             option {
//                                                 value: metadata_type.clone(),
//                                                 selected: metadata_type == selected_type(),
//                                                 "{metadata_type.clone()}"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                             div { class: "flex flex-row w-full justify-start items-center  mb-[10px]",
//                                 div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
//                                     "{translate.field}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
//                                     select {
//                                         class: "bg-transparent focus:outline-none w-full",
//                                         value: selected_field(),
//                                         onchange: move |e| {
//                                             selected_field.set(e.value());
//                                         },
//                                         option {
//                                             value: "",
//                                             disabled: true,
//                                             selected: selected_field() == translate.no_selection.to_string(),
//                                             "{translate.no_selection}"
//                                         }
//                                         for metadata_field in total_fields.clone() {
//                                             option {
//                                                 value: metadata_field.clone(),
//                                                 selected: metadata_field == selected_field(),
//                                                 "{metadata_field.clone()}"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                             div { class: "flex flex-row w-full justify-start items-center  mb-[10px]",
//                                 div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
//                                     "{translate.purpose_of_use}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
//                                     select {
//                                         class: "bg-transparent focus:outline-none w-full",
//                                         value: selected_purpose(),
//                                         onchange: move |e| {
//                                             selected_purpose.set(e.value());
//                                         },
//                                         option {
//                                             value: "",
//                                             disabled: true,
//                                             selected: selected_purpose() == translate.no_selection.to_string(),
//                                             "{translate.no_selection}"
//                                         }
//                                         for metadata_purpose in total_purposes.clone() {
//                                             option {
//                                                 value: metadata_purpose.clone(),
//                                                 selected: metadata_purpose == selected_purpose(),
//                                                 "{metadata_purpose.clone()}"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                             div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
//                                 div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
//                                     "{translate.source}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
//                                     select {
//                                         class: "bg-transparent focus:outline-none w-full",
//                                         value: selected_source(),
//                                         onchange: move |e| {
//                                             selected_source.set(e.value());
//                                         },
//                                         option {
//                                             value: "",
//                                             disabled: true,
//                                             selected: selected_source() == translate.no_selection.to_string(),
//                                             "{translate.no_selection}"
//                                         }
//                                         for metadata_source in total_resources.clone() {
//                                             option {
//                                                 value: metadata_source.clone(),
//                                                 selected: metadata_source == selected_source(),
//                                                 "{metadata_source.clone()}"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                             div { class: "flex flex-row w-full justify-start items-center",
//                                 div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
//                                     "{translate.permissions}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
//                                     select {
//                                         class: "bg-transparent focus:outline-none w-full",
//                                         value: selected_authority(),
//                                         onchange: move |e| {
//                                             selected_authority.set(e.value());
//                                         },
//                                         option {
//                                             value: "",
//                                             disabled: true,
//                                             selected: selected_authority() == translate.no_selection.to_string(),
//                                             "{translate.no_selection}"
//                                         }
//                                         for metadata_authority in total_authorities.clone() {
//                                             option {
//                                                 value: metadata_authority.clone(),
//                                                 selected: metadata_authority == selected_authority(),
//                                                 "{metadata_authority.clone()}"
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }

//                     div { class: "flex flex-col w-full justify-start items-start",
//                         div { class: "font-medium text-[#222222] text-[15px] mb-[10px]",
//                             "{translate.link_to_survey}"
//                         }

//                         div { class: "flex flex-col w-full justify-start items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px] mb-[10px]",
//                             div { class: "flex flex-row w-full justify-start items-center",
//                                 div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
//                                     "{translate.public_opinion}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
//                                     div { class: "font-medium text-[15px] text-[#b4b4b4]",
//                                         "{translate.input_keyword}"
//                                     }
//                                 }
//                             }
//                             div { class: "flex flex-row w-full justify-start items-center mt-[10px]",
//                                 div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
//                                     "{translate.survey}"
//                                 }
//                                 div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
//                                     div { class: "font-medium text-[15px] text-[#b4b4b4]",
//                                         "{translate.input_keyword}"
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             //FIXME: fix to real data
//             div { class: "flex flex-row w-full justify-start items-start font-normal text-[#6d6d6d] text-[14px] mt-[40px] mb-[20px]",
//                 "총 5개 자료 업로드"
//             }
//             //FIXME: fix to connect project
//             div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
//                 button {
//                     class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
//                     onclick: move |_| {
//                         onupload
//                             .call(CreateMetadataRequest {
//                                 name: name(),
//                                 urls: metadata_urls(),
//                                 metadata_type: if selected_type() == translate.no_selection {
//                                     None
//                                 } else {
//                                     Some(selected_type().parse().unwrap())
//                                 },
//                                 metadata_field: if selected_field() == translate.no_selection {
//                                     None
//                                 } else {
//                                     Some(selected_field().parse().unwrap())
//                                 },
//                                 metadata_purpose: if selected_purpose() == translate.no_selection {
//                                     None
//                                 } else {
//                                     Some(selected_purpose().parse().unwrap())
//                                 },
//                                 metadata_source: if selected_source() == translate.no_selection {
//                                     None
//                                 } else {
//                                     Some(selected_source().parse().unwrap())
//                                 },
//                                 metadata_authority: if selected_authority() == translate.no_selection {
//                                     None
//                                 } else {
//                                     Some(selected_authority().parse().unwrap())
//                                 },
//                                 public_opinion_projects: None,
//                                 public_survey_projects: None,
//                             });
//                     },
//                     Upload { width: "24", height: "24" }
//                     div { class: "text-white font-semibold text-[#16px]", "{translate.upload}" }
//                 }
//                 button {
//                     class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
//                     onclick: move |e: Event<MouseData>| {
//                         onclose.call(e);
//                     },
//                     "{translate.cancel}"
//                 }
//             }
//         }
//     }
// }
