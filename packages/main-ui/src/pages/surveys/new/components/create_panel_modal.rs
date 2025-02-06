use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2},
    PanelV2CreateRequest,
};

use crate::{
    components::icons::{Clear, Remove},
    pages::surveys::new::i18n::CreatePanelModalTranslate,
};

#[derive(Props, Clone, PartialEq)]
pub struct CreatePanelModalProps {
    lang: Language,
    onsave: EventHandler<PanelV2CreateRequest>,
    oncancel: EventHandler<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub name: String,
    pub values: Vec<String>,
}

#[component]
pub fn CreatePanelModal(props: CreatePanelModalProps) -> Element {
    let translate: CreatePanelModalTranslate = translate(&props.lang);
    let mut panel_name: Signal<String> = use_signal(|| "".to_string());
    let mut panel_count: Signal<u64> = use_signal(|| 0);
    let mut selected_value: Signal<Vec<String>> = use_signal(|| {
        vec![
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]
    });
    let mut is_open: Signal<Vec<bool>> = use_signal(|| vec![false, false, false, false]);

    let panel_name_error: Signal<String> = use_signal(|| "".to_string());
    let panel_count_error: Signal<String> = use_signal(|| "".to_string());
    let error_signals: Vec<Signal<String>> = vec![
        use_signal(|| "".to_string()), // age_error
        use_signal(|| "".to_string()), // gender_error
        use_signal(|| "".to_string()), // region_error
        use_signal(|| "".to_string()), // salary_error
    ];

    let total_attributes: Signal<Vec<AttributeInfo>> = use_signal(|| {
        vec![
            AttributeInfo {
                name: translate.clone().age.to_string(),
                values: vec![
                    translate.clone().teenager.to_string(),
                    translate.clone().twenty.to_string(),
                    translate.clone().thirty.to_string(),
                    translate.clone().fourty.to_string(),
                    translate.clone().fifty.to_string(),
                    translate.clone().sixty.to_string(),
                    translate.clone().over.to_string(),
                ],
            },
            AttributeInfo {
                name: translate.clone().gender.to_string(),
                values: vec![
                    translate.clone().male.to_string(),
                    translate.clone().female.to_string(),
                ],
            },
            AttributeInfo {
                name: translate.clone().region.to_string(),
                values: vec![
                    translate.clone().seoul.to_string(),
                    translate.clone().busan.to_string(),
                    translate.clone().daegu.to_string(),
                    translate.clone().incheon.to_string(),
                    translate.clone().gwangju.to_string(),
                    translate.clone().daejeon.to_string(),
                    translate.clone().ulsan.to_string(),
                    translate.clone().sejong.to_string(),
                    translate.clone().gyeongi.to_string(),
                    translate.clone().gangwon.to_string(),
                    translate.clone().chungbuk.to_string(),
                    translate.clone().chungnam.to_string(),
                    translate.clone().jeonbuk.to_string(),
                    translate.clone().jeonnam.to_string(),
                    translate.clone().gyeonbuk.to_string(),
                    translate.clone().gyeonnam.to_string(),
                    translate.clone().jeju.to_string(),
                ],
            },
            AttributeInfo {
                name: translate.clone().salary.to_string(),
                values: vec![
                    translate.clone().tier_one.to_string(),
                    translate.clone().tier_two.to_string(),
                    translate.clone().tier_three.to_string(),
                    translate.clone().tier_four.to_string(),
                    translate.clone().tier_five.to_string(),
                ],
            },
        ]
    });

    rsx! {
        div { class: "flex flex-col w-[540px] justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
                div { class: "font-semibold text-[#222222] text-[14px] leading-[22.5px] mb-[15px]",
                    "{translate.panel_name}"
                }
                input {
                    class: "flex flex-row w-full justify-start items-center focus:outline-none px-[15px] py-[10px] bg-[#f7f7f7] rounded-[4px] font-medium text-[15px] text-[#b4b4b4] mb-[5px]",
                    r#type: "text",
                    placeholder: "{translate.input_panel_name}",
                    value: (panel_name)(),
                    oninput: move |event| {
                        panel_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[#222222] text-[13px]",
                    "{translate.input_panel_name_description}"
                }

                if panel_name_error() != "" {
                    div { class: "font-semibold text-red-600 text-[13px] mt-[10px]",
                        {panel_name_error()}
                    }
                }
            }

            div { class: "flex flex-col w-full justify-start items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px]",
                div { class: "flex flex-row w-full justify-start items-center gap-[10px]",
                    div { class: "w-[50px] font-medium text-[#222222] text-[15px]",
                        "{translate.personnel}"
                    }
                    div { class: "flex flex-col w-full justify-start items-start",
                        input {
                            class: "flex flex-row w-full h-[55px] justify-end items-center rounded-[4px] px-[15px] py-[10px] bg-[#f7f7f7] font-medium text-[#222222] text-[15px] focus:outline-none",
                            r#type: "text",
                            placeholder: "0",
                            value: panel_count(),
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
                                move |e: Event<FormData>| {
                                    let value = e.value().parse::<u64>().unwrap_or(0);
                                    panel_count.set(value);
                                }
                            },
                        }

                        if panel_count_error() != "" {
                            div { class: "font-semibold text-red-600 text-[13px] mt-[10px]",
                                {panel_count_error()}
                            }
                        }
                    }
                }

                for (index , attribute) in total_attributes().iter().enumerate() {
                    div { class: "flex flex-row w-full justify-start items-center gap-[10px] mt-[10px]",
                        div { class: "w-[50px] font-medium text-[#222222] text-[15px]",
                            "{attribute.name}"
                        }
                        div { class: "relative w-full",
                            div { class: "flex flex-col w-full justify-start items-start",
                                button {
                                    class: "flex flex-row w-full justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px] min-h-[55px]",
                                    onclick: move |_| {
                                        let mut open = is_open();
                                        open[index] = true;
                                        is_open.set(open);
                                    },
                                    if selected_value()[index] != "" {
                                        AttributeLabel {
                                            label: selected_value()[index].clone(),
                                            onclose: move |e: Event<MouseData>| {
                                                e.stop_propagation();
                                                e.prevent_default();
                                                let mut attributes = selected_value();
                                                attributes[index] = "".to_string();
                                                selected_value.set(attributes);
                                            },
                                        }
                                    }
                                }

                                if error_signals[index]() != "" {
                                    div { class: "font-semibold text-red-600 text-[13px] mt-[10px]",
                                        {error_signals[index]()}
                                    }
                                }
                            }

                            if is_open()[index] {
                                div { class: "absolute flex flex-col w-full justify-start items-center shadow-[0px_8px_20px_rgba(20,26,62,0.25)] bg-white py-4 rounded-md z-20",
                                    div { class: "flex flex-row w-full justify-end px-[10px]",
                                        button {
                                            onclick: move |e: Event<MouseData>| {
                                                e.stop_propagation();
                                                e.prevent_default();
                                                let mut open = is_open();
                                                open[index] = false;
                                                is_open.set(open);
                                            },
                                            Remove {
                                                width: "15",
                                                height: "15",
                                                fill: "#555462",
                                            }
                                        }
                                    }

                                    div { class: "flex flex-col w-full max-h-[150px] overflow-y-auto justify-start items-start",
                                        for value in attribute.values.clone() {
                                            div {
                                                class: "flex flex-col w-full h-[60px] justify-start items-start py-[9px] bg-white hover:bg-[#f7f7f7] hover:border-l hover:border-l-[#2a60d3] cursor-pointer",
                                                onclick: move |_| {
                                                    tracing::info!("attribute value: {}", value);
                                                    let mut values = selected_value();
                                                    values[index] = value.clone();
                                                    selected_value.set(values);
                                                    let mut open = is_open();
                                                    open[index] = false;
                                                    is_open.set(open);
                                                },
                                                div { class: "flex flex-col w-full px-4",
                                                    div { class: "font-bold text-[15px] text-[#222222] mb-[5px]",
                                                        "{value}"
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

            div { class: "flex flex-row w-full justify-start items-center gap-[20px] mt-[40px]",
                //FIXME: add error handling
                button {
                    class: "flex flex-row bg-[#2a60d3] rounded-[4px] px-[14px] py-[8px] font-semibold text-white text-[16px] leading-[24px]",
                    onclick: {
                        let values = selected_value();
                        let translate = translate.clone();
                        let mut age: Option<AgeV2> = None;
                        let mut gender: Option<GenderV2> = None;
                        let mut region: Option<RegionV2> = None;
                        let mut salary: Option<SalaryV2> = None;
                        if values[0] != "" && values[1] != "" && values[2] != "" && values[3] != "" {
                            age = Some(values[0].parse().unwrap());
                            gender = Some(values[1].parse().unwrap());
                            region = Some(values[2].parse().unwrap());
                            salary = Some(values[3].parse().unwrap());
                        }
                        move |_| {
                            if check_condition(
                                translate.clone(),
                                panel_name_error,
                                panel_count_error,
                                error_signals[0],
                                error_signals[1],
                                error_signals[2],
                                error_signals[3],
                                panel_name(),
                                panel_count(),
                                values[0].clone(),
                                values[1].clone(),
                                values[2].clone(),
                                values[3].clone(),
                            ) {
                                props
                                    .onsave
                                    .call(PanelV2CreateRequest {
                                        name: panel_name(),
                                        user_count: panel_count(),
                                        age: age.clone().unwrap(),
                                        gender: gender.clone().unwrap(),
                                        region: region.clone().unwrap(),
                                        salary: salary.clone().unwrap(),
                                    });
                            }
                        }
                    },
                    "{translate.save}"
                }
                button {
                    class: "flex flex-row bg-white px-[14px] py-[8px] font-semibold text-[#222222] text-[16px] leading-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        props.oncancel.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn AttributeLabel(label: String, onclose: EventHandler<MouseEvent>) -> Element {
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

pub fn check_condition(
    tr: CreatePanelModalTranslate,

    mut panel_name_error: Signal<String>,
    mut panel_count_error: Signal<String>,
    mut age_error: Signal<String>,
    mut gender_error: Signal<String>,
    mut region_error: Signal<String>,
    mut salary_error: Signal<String>,

    panel_name: String,
    panel_count: u64,
    age: String,
    gender: String,
    region: String,
    salary: String,
) -> bool {
    if panel_name.len() < 2 {
        panel_name_error.set(tr.panel_name_error.to_string());
        return false;
    } else {
        panel_name_error.set("".to_string());
    }

    if panel_count == 0 {
        panel_count_error.set(tr.panel_count_error.to_string());
        return false;
    } else {
        panel_count_error.set("".to_string());
    }

    if age == "" {
        age_error.set(tr.age_error.to_string());
        return false;
    } else {
        age_error.set("".to_string());
    }

    if gender == "" {
        gender_error.set(tr.gender_error.to_string());
        return false;
    } else {
        gender_error.set("".to_string());
    }

    if region == "" {
        region_error.set(tr.region_error.to_string());
        return false;
    } else {
        region_error.set("".to_string());
    }

    if salary == "" {
        salary_error.set(tr.salary_error.to_string());
        return false;
    } else {
        salary_error.set("".to_string());
    }

    true
}
