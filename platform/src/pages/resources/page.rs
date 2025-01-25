use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::CreateMetadataRequest;

use crate::{
    components::{
        icons::{ArrowLeft, Edit, RowOption, Search, Switch, Upload, UploadFile},
        upload_button::UploadButton,
    },
    pages::resources::{
        controller::Controller,
        i18n::{
            CreateMaterialModalTranslate, DirectUploadedTranslate, RemoveMaterialModalTranslate,
            ResourceTranslate, UploadMaterialModalTranslate,
        },
    },
    service::popup_service::PopupService,
};

#[cfg(feature = "web")]
use dioxus::html::HasFileData;

#[cfg(feature = "web")]
use web_sys::window;

#[derive(Props, Clone, PartialEq)]
pub struct ResourceProps {
    lang: Language,
}

#[component]
pub fn ResourcePage(props: ResourceProps) -> Element {
    let popup_service: PopupService = use_context();
    let ctrl = Controller::new(props.lang, popup_service);
    let translate: ResourceTranslate = translate(&props.lang);
    let mut is_focused = use_signal(|| false);
    let mut resource_name = use_signal(|| "".to_string());
    let resources = ctrl.get_resources();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                    "{translate.resource_title}"
                }
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translate.resource_title}"
            }
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
                        button {
                            onclick: move |_| {
                                ctrl.open_create_material(props.lang);
                            },
                            div { class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                                Upload { width: "24", height: "24" }
                                div { class: "text-white font-semibold text-[#16px]",
                                    "{translate.upload_material}"
                                }
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

                        for (index , resource) in resources.clone().iter().enumerate() {
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
                                        if resource.public_opinion_projects.clone().unwrap_or_default().len() > 1 {
                                            MetadataLabel { label: resource.public_opinion_projects.clone().unwrap()[0].name.clone() }
                                        } else if resource.public_survey_projects.clone().unwrap_or_default().len() > 1 {
                                            MetadataLabel { label: resource.public_survey_projects.clone().unwrap()[0].name.clone() }
                                        }
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
                                        button {
                                            class: "text-[#2a60d3] font-semibold text-[14px]",
                                            onclick: {
                                                let resource = resource.clone();
                                                move |_| {
                                                    #[cfg(feature = "web")]
                                                    {
                                                        for link in resource.urls.clone() {
                                                            if let Some(win) = window() {
                                                                win.open_with_url_and_target(&link, "_blank").ok();
                                                            }
                                                        }
                                                    }
                                                    #[cfg(not(feature = "web"))]
                                                    {
                                                        let _ = &resource;
                                                    }
                                                }
                                            },
                                            "{translate.download}"
                                        }
                                    }
                                    div { class: "group relative",
                                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                            button {
                                                RowOption { width: "24", height: "24" }
                                            }
                                            nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                ul { class: "py-1",
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| {
                                                            ctrl.open_update_material(props.lang, index);
                                                        },
                                                        "{translate.update_material_li}"
                                                    }
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| {
                                                            ctrl.open_remove_material(props.lang, index);
                                                        },
                                                        "{translate.remove_material_li}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //pagenation
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
}

#[component]
pub fn UpdateMaterialModal(
    lang: Language,
    initial_title: String,
    onupload: EventHandler<(String, Vec<String>)>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: UploadMaterialModalTranslate = translate(&lang);
    let mut title: Signal<String> = use_signal(|| initial_title);
    let metadata_urls: Signal<Vec<String>> = use_signal(|| vec![]);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#6d6d6d] font-normal text-[14px] mb-[40px]",
                "{translate.upload_material_modal_description}"
            }

            div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
                    "{translate.input_title}"
                }
                div {
                    class: format!(
                        "flex flex-row w-full h-[45px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mb-[5px]",
                    ),
                    input {
                        class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
                        r#type: "text",
                        value: title(),
                        placeholder: translate.input_hint,
                        oninput: move |e| {
                            title.set(e.value());
                        },
                    }
                }
                div { class: "font-normal text-[#222222] text-[13px]", "{translate.input_info}" }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
                    "{translate.classification}"
                }
                DirectUpload { lang, metadata_urls }
            }

            div { class: "flex flex-row w-full justify-start items-start font-normal text-[#6d6d6d] text-[14px] mt-[40px] mb-[20px]",
                "총 5개 자료 업로드"
            }
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                button {
                    class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                    onclick: move |_| async move {
                        onupload.call((title(), metadata_urls()));
                    },
                    Edit { width: "24", height: "24" }
                    div { class: "text-white font-semibold text-[#16px]", "{translate.update}" }
                }
                button {
                    class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn CreateMaterialModal(
    lang: Language,
    onupload: EventHandler<CreateMetadataRequest>,
    onclose: EventHandler<MouseEvent>,

    total_types: Vec<String>,
    total_fields: Vec<String>,
    total_purposes: Vec<String>,
    total_resources: Vec<String>,
    total_authorities: Vec<String>,
) -> Element {
    let translate: CreateMaterialModalTranslate = translate(&lang);

    let mut selected_type = use_signal(|| translate.no_selection.to_string());
    let mut selected_field = use_signal(|| translate.no_selection.to_string());
    let mut selected_purpose = use_signal(|| translate.no_selection.to_string());
    let mut selected_source = use_signal(|| translate.no_selection.to_string());
    let mut selected_authority = use_signal(|| translate.no_selection.to_string());

    let metadata_urls: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut name: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#6d6d6d] font-normal text-[14px] mb-[40px]",
                "{translate.create_material_modal_translate}"
            }
            div { class: "flex flex-row w-full justify-start items-start gap-[40px]",
                div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start mr-[40px]",
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
                            "{translate.input_title}"
                        }
                        div {
                            class: format!(
                                "flex flex-row w-full h-[45px] justify-start items-center p-[15px] bg-[#f7f7f7] rounded-[4px] mb-[5px]",
                            ),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-[#b4b4b4] placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-[#222222]",
                                r#type: "text",
                                value: name(),
                                placeholder: translate.input_hint,
                                oninput: move |e| {
                                    name.set(e.value());
                                },
                            }
                        }
                        div { class: "font-normal text-[#222222] text-[13px]", "{translate.input_info}" }
                    }

                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
                            "{translate.classification}"
                        }
                        DirectUpload { lang, metadata_urls }
                    }
                }

                div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start gap-[40px]",
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "font-semibold text-[#222222] text-[14px] mb-[10px]",
                            "{translate.classification}"
                        }

                        div { class: "flex flex-col w-full justify-start items-start p-[24px] bg-white border border-[#bfc8d9] rounded-[8px]",
                            div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                                div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
                                    "{translate.metadata_type}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
                                    select {
                                        class: "bg-transparent focus:outline-none w-full",
                                        value: selected_type(),
                                        onchange: move |e| {
                                            selected_type.set(e.value());
                                        },
                                        option {
                                            value: "",
                                            disabled: true,
                                            selected: selected_type() == translate.no_selection.to_string(),
                                            "{translate.no_selection}"
                                        }
                                        for metadata_type in total_types.clone() {
                                            option {
                                                value: metadata_type.clone(),
                                                selected: metadata_type == selected_type(),
                                                "{metadata_type.clone()}"
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center  mb-[10px]",
                                div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
                                    "{translate.field}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
                                    select {
                                        class: "bg-transparent focus:outline-none w-full",
                                        value: selected_field(),
                                        onchange: move |e| {
                                            selected_field.set(e.value());
                                        },
                                        option {
                                            value: "",
                                            disabled: true,
                                            selected: selected_field() == translate.no_selection.to_string(),
                                            "{translate.no_selection}"
                                        }
                                        for metadata_field in total_fields.clone() {
                                            option {
                                                value: metadata_field.clone(),
                                                selected: metadata_field == selected_field(),
                                                "{metadata_field.clone()}"
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center  mb-[10px]",
                                div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
                                    "{translate.purpose_of_use}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
                                    select {
                                        class: "bg-transparent focus:outline-none w-full",
                                        value: selected_purpose(),
                                        onchange: move |e| {
                                            selected_purpose.set(e.value());
                                        },
                                        option {
                                            value: "",
                                            disabled: true,
                                            selected: selected_purpose() == translate.no_selection.to_string(),
                                            "{translate.no_selection}"
                                        }
                                        for metadata_purpose in total_purposes.clone() {
                                            option {
                                                value: metadata_purpose.clone(),
                                                selected: metadata_purpose == selected_purpose(),
                                                "{metadata_purpose.clone()}"
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                                div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
                                    "{translate.source}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
                                    select {
                                        class: "bg-transparent focus:outline-none w-full",
                                        value: selected_source(),
                                        onchange: move |e| {
                                            selected_source.set(e.value());
                                        },
                                        option {
                                            value: "",
                                            disabled: true,
                                            selected: selected_source() == translate.no_selection.to_string(),
                                            "{translate.no_selection}"
                                        }
                                        for metadata_source in total_resources.clone() {
                                            option {
                                                value: metadata_source.clone(),
                                                selected: metadata_source == selected_source(),
                                                "{metadata_source.clone()}"
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center",
                                div { class: "flex flex-row w-[90px] justify-start items-start mr-[50px]",
                                    "{translate.permissions}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center h-[45px] bg-[#f7f7f7] rounded-[4px]",
                                    select {
                                        class: "bg-transparent focus:outline-none w-full",
                                        value: selected_authority(),
                                        onchange: move |e| {
                                            selected_authority.set(e.value());
                                        },
                                        option {
                                            value: "",
                                            disabled: true,
                                            selected: selected_authority() == translate.no_selection.to_string(),
                                            "{translate.no_selection}"
                                        }
                                        for metadata_authority in total_authorities.clone() {
                                            option {
                                                value: metadata_authority.clone(),
                                                selected: metadata_authority == selected_authority(),
                                                "{metadata_authority.clone()}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "font-medium text-[#222222] text-[15px] mb-[10px]",
                            "{translate.link_to_survey}"
                        }

                        div { class: "flex flex-col w-full justify-start items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px] mb-[10px]",
                            div { class: "flex flex-row w-full justify-start items-center",
                                div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
                                    "{translate.public_opinion}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
                                    div { class: "font-medium text-[15px] text-[#b4b4b4]",
                                        "{translate.input_keyword}"
                                    }
                                }
                            }
                            div { class: "flex flex-row w-full justify-start items-center mt-[10px]",
                                div { class: "font-medium text-[#3a3a3a] text-[15px] mr-[10px] w-[50px]",
                                    "{translate.survey}"
                                }
                                div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px]",
                                    div { class: "font-medium text-[15px] text-[#b4b4b4]",
                                        "{translate.input_keyword}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            //FIXME: fix to real data
            div { class: "flex flex-row w-full justify-start items-start font-normal text-[#6d6d6d] text-[14px] mt-[40px] mb-[20px]",
                "총 5개 자료 업로드"
            }
            //FIXME: fix to connect project
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                button {
                    class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                    onclick: move |_| {
                        onupload
                            .call(CreateMetadataRequest {
                                name: name(),
                                urls: metadata_urls(),
                                metadata_type: if selected_type() == translate.no_selection {
                                    None
                                } else {
                                    Some(selected_type().parse().unwrap())
                                },
                                metadata_field: if selected_field() == translate.no_selection {
                                    None
                                } else {
                                    Some(selected_field().parse().unwrap())
                                },
                                metadata_purpose: if selected_purpose() == translate.no_selection {
                                    None
                                } else {
                                    Some(selected_purpose().parse().unwrap())
                                },
                                metadata_source: if selected_source() == translate.no_selection {
                                    None
                                } else {
                                    Some(selected_source().parse().unwrap())
                                },
                                metadata_authority: if selected_authority() == translate.no_selection {
                                    None
                                } else {
                                    Some(selected_authority().parse().unwrap())
                                },
                                public_opinion_projects: None,
                                public_survey_projects: None,
                            });
                    },
                    Upload { width: "24", height: "24" }
                    div { class: "text-white font-semibold text-[#16px]", "{translate.upload}" }
                }
                button {
                    class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn DirectUpload(lang: Language, metadata_urls: Signal<Vec<String>>) -> Element {
    let mut indragzone = use_signal(|| false);
    let translate: DirectUploadedTranslate = translate(&lang);

    rsx! {
        div {
            class: "flex flex-col w-full",
            ondrop: move |ev: Event<DragData>| async move {
                tracing::debug!("drop files in div");
                ev.prevent_default();
                ev.stop_propagation();
            },
            div {
                class: format!(
                    "flex flex-col w-full justify-center items-center p-[24px] rounded-[8px] border-[2px] border-dashed border-[#2a60d3] mb-[10px] {}",
                    if indragzone() { "bg-[#afc9ff] opacity-50" } else { "" },
                ),
                ondragover: move |e| {
                    e.prevent_default();
                    e.stop_propagation();
                    tracing::debug!("files in drop zone");
                    indragzone.set(true);
                },
                ondragleave: move |e| {
                    e.prevent_default();
                    e.stop_propagation();
                    tracing::debug!("leave drop zone");
                    indragzone.set(false);
                },
                //TODO: add file upload code
                ondrop: move |ev: Event<DragData>| async move {
                    tracing::debug!("drop files");
                    ev.prevent_default();
                    ev.stop_propagation();
                    #[cfg(feature = "web")]
                    if let Some(file_engine) = ev.files() {
                        tracing::debug!("got file_engine {:?}", file_engine.files());
                    }
                    indragzone.set(false);
                },
                div { class: "mb-[12px] w-[42px] h-[42px]",
                    UploadFile { width: "42", height: "42" }
                }
                div { class: "font-normal text-[#222222] text-sm mb-[8px]",
                    "{translate.direct_upload_description}"
                }
                div { class: "flex flex-row w-full justify-center items-center mb-[8px]",
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                    div { class: "font-normal text-[#6d6d6d] text-sm mr-[12px]", "OR" }
                    div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                }
                //TODO: add file upload code
                UploadButton {
                    class: "flex flex-row w-[100px] h-[30px] justify-center items-center bg-white border border-[#1849d6] rounded-[4px] font-semibold text-[#1849d6] text-sm",
                    text: "{translate.load_file}",
                    onuploaded: move |_| {},
                }
            }

            div { class: "font-normal text-[#6d6d6d] text-[14px]", "{translate.load_file_info}" }
        }
    }
}

#[component]
pub fn RemoveMaterialModal(
    lang: Language,
    onremove: EventHandler<MouseEvent>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let translate: RemoveMaterialModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{translate.remove_material_modal_title}" }
                div { "{translate.remove_material_modal_description}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    div {
                        class: "text-white font-bold text-[16px]",
                        onclick: move |e: MouseEvent| {
                            onremove.call(e);
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
pub fn MetadataLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[100px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
