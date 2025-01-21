use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{ArrowLeft, RowOption, Search, Switch, Upload},
    pages::resources::{controller::Controller, i18n::ResourceTranslate},
};

#[derive(Props, Clone, PartialEq)]
pub struct ResourceProps {
    lang: Language,
}

#[component]
pub fn ResourcePage(props: ResourceProps) -> Element {
    let ctrl = Controller::new(props.lang);
    let translate: ResourceTranslate = translate(&props.lang);
    let mut is_focused = use_signal(|| false);
    let mut resource_name = use_signal(|| "".to_string());
    let resources = ctrl.get_resources();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translate.resource_title}"
            }
        }
        div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]", "{translate.resource_title}" }
        div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
            "{translate.resource_description}"
        }

        div { class: "flex flex-col w-full justify-start items-start mb-[50px]",
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
                            value: (resource_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                resource_name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                        Upload { width: "24", height: "24" }
                        div { class: "text-white font-semibold text-[#16px]",
                            "{translate.upload_material}"
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.metadata_type}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.field}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[180px] min-w-[180px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.purpose}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.title}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.linked_surveys}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.source}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.authority}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.last_modified_date}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.function}"
                            }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }

                    for (_index , resource) in resources.iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        if resource.metadata_type.is_none() {
                                            "{translate.not_exists}"
                                        } else {
                                            {
                                                ctrl.translate_metadata_type(props.lang, resource.metadata_type.clone().unwrap())
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        if resource.metadata_field.is_none() {
                                            "{translate.not_exists}"
                                        } else {
                                            {
                                                ctrl.translate_metadata_field(
                                                    props.lang,
                                                    resource.metadata_field.clone().unwrap(),
                                                )
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[180px] min-w-[180px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        if resource.metadata_purpose.is_none() {
                                            "{translate.not_exists}"
                                        } else {
                                            {
                                                ctrl.translate_metadata_purpose(
                                                    props.lang,
                                                    resource.metadata_purpose.clone().unwrap(),
                                                )
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        "{resource.name.clone()}"
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    //FIXME: fix to real public opinion name
                                    MetadataLabel { label: "공론명" }
                                }
                                div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        if resource.metadata_source.is_none() {
                                            "{translate.not_exists}"
                                        } else {
                                            {
                                                ctrl.translate_metadata_source(
                                                    props.lang,
                                                    resource.metadata_source.clone().unwrap(),
                                                )
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        if resource.metadata_authority.is_none() {
                                            "{translate.not_exists}"
                                        } else {
                                            {
                                                ctrl.translate_metadata_authority(
                                                    props.lang,
                                                    resource.metadata_authority.clone().unwrap(),
                                                )
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                    div { class: "text-[#555462] font-semibold text-[14px]",
                                        {ctrl.convert_timestamp_to_date(resource.updated_at)}
                                    }
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                    div { class: "text-[#2a60d3] font-semibold text-[14px]",
                                        "{translate.download}"
                                    }
                                }
                                div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                    RowOption { width: "24", height: "24" }
                                }
                            }
                        }
                    }
                }

                //페이지네이션
                div { class: "flex flex-row w-full justify-center items-center mt-[20px]",
                    div { class: "mr-[20px] w-[24px] h-[24px]",
                        ArrowLeft { width: "24", height: "24" }
                    }
                    //FIXME: add pagination by variable(page, index)
                    for i in 0..10 {
                        if i == 0 {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        } else {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        }
                    }
                    div { class: "flex flex-row ml-[12px] w-[60px] h-[40px] justify-center items-center font-bold text-[15px] text-[#0d1732]",
                        "More"
                    }
                }
            }
        }
    }
}

#[component]
pub fn MetadataLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[100px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
