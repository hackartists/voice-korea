#![allow(non_snake_case)]
use crate::pages::groups::controller::Controller;
use crate::pages::groups::i18n::CreateGroupModalTranslate;
use crate::pages::groups::i18n::RemoveGroupModalTranslate;
use crate::pages::groups::i18n::UpdateGroupNameModalTranslate;
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::GroupTranslate;
use models::prelude::CreateGroupRequest;

use crate::{
    components::{
        icons::{ArrowLeft, ArrowRight, Expand, Folder, RowOption, Search, Switch},
        label::Label,
    },
    routes::Route,
    service::popup_service::PopupService,
};

use super::i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupPageProps {
    lang: Language,
}

#[component]
pub fn GroupPage(props: GroupPageProps) -> Element {
    let popup: PopupService = use_context();
    let ctrl = Controller::init(props.lang, popup);
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let translates: GroupTranslate = translate(&props.lang);

    let mut clicked_group_id = use_signal(|| "".to_string());
    let mut clicked_group_name = use_signal(|| "".to_string());

    let group = ctrl.get_groups();
    let groups = group.clone();
    let group_len = groups.len();

    let members = ctrl.get_members();

    let mut member_clicked = use_signal(|| vec![]);
    let mut member_extended = use_signal(|| vec![]);
    let mut member_add_extended = use_signal(|| vec![]);

    use_effect(use_reactive(&group_len, move |group_len| {
        member_clicked.set(vec![false; group_len]);
        member_extended.set(vec![false; group_len]);
        member_add_extended.set(vec![false; group_len]);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.group_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.group_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translates.group_description}"
            }

            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
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
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        button {
                            class: "flex flex-row w-[140px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            onclick: move |_| async move {
                                ctrl.open_create_group_modal(props.lang, clicked_group_id, clicked_group_name)
                                    .await;
                            },
                            Folder { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]",
                                "{translates.create_group}"
                            }
                        }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.personnel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.team_member}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for index in 0..groups.len() {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    Link {
                                        to: Route::GroupDetailPage {
                                            lang: props.lang.clone(),
                                            group_id: groups[index].group_id.clone(),
                                        },
                                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center",
                                            "{groups[index].group_name}"
                                        }
                                    }
                                    div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                        "{groups[index].member_count}"
                                    }
                                    div {
                                        class: "flex flex-row w-full h-full justify-center items-center cursor-pointer relative",
                                        onclick: move |_| {
                                            let mut clicked = member_clicked.clone()();
                                            clicked[index] = !clicked[index];
                                            member_clicked.set(clicked);
                                        },
                                        if groups.len() != 0 && index < member_clicked().len()
                                            && (!member_clicked()[index] && groups[index].member_list.len() > 0)
                                        {
                                            Label {
                                                label_name: if groups[index].member_list[0].clone().name != "" { groups[index].member_list[0].clone().name } else { groups[index].member_list[0].clone().email },
                                                label_color: "bg-[#35343f]",
                                            }
                                        } else {
                                            if groups.len() != 0 {
                                                div { class: "flex flex-row w-full h-full",
                                                    div { class: "flex flex-row w-full justify-center items-center",
                                                        div { class: "inline-flex flex-wrap justify-center items-center gap-[10px] mr-[20px]",
                                                            for member in groups[index].member_list.clone() {
                                                                Label {
                                                                    label_name: if member.name != "" { member.name } else { member.email },
                                                                    label_color: "bg-[#35343f]",
                                                                }
                                                            }
                                                        }
                                                        div {
                                                            class: "flex flex-row mr-[20px] w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] rounded-[4px] text-[15px] font-bold text-[#35343f]",
                                                            onclick: move |e: MouseEvent| {
                                                                e.stop_propagation();
                                                                e.prevent_default();
                                                                let mut extended = member_add_extended.clone()();
                                                                extended[index] = !extended[index];
                                                                member_add_extended.set(extended);
                                                                let mut extended = member_extended.clone()();
                                                                extended[index] = false;
                                                                member_extended.set(extended);
                                                            },
                                                            "+"
                                                        }
                                                        div {
                                                            onclick: move |e: MouseEvent| {
                                                                e.stop_propagation();
                                                                e.prevent_default();
                                                                let mut extended = member_extended.clone()();
                                                                extended[index] = !extended[index];
                                                                member_extended.set(extended);
                                                                let mut extended = member_add_extended.clone()();
                                                                extended[index] = false;
                                                                member_add_extended.set(extended);
                                                            },
                                                            Expand {
                                                                width: "24",
                                                                height: "24",
                                                            }
                                                        }
                                                    }
                                                    if index < member_extended().len() && member_extended()[index] {
                                                        div { class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50 py-[20px] pl-[15px] pr-[100px]",
                                                            div { class: "font-semibold text-[#7c8292] text-[14px] mb-[20px]",
                                                                "{translates.team_member}"
                                                            }
                                                            div { class: "inline-flex flex-wrap justify-start items-start gap-[10px] mr-[20px]",
                                                                for member in groups[index].member_list.clone() {
                                                                    Label {
                                                                        label_name: if member.name != "" { member.name } else { member.email },
                                                                        label_color: "bg-[#35343f]",
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    if index < member_add_extended().len() && member_add_extended()[index] {
                                                        div {
                                                            class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-[4px] w-full z-50",
                                                            onclick: move |event| {
                                                                event.stop_propagation();
                                                                event.prevent_default();
                                                            },
                                                            div { class: "flex flex-col w-full justify-start items-start",
                                                                div {
                                                                    class: format!(
                                                                        "flex flex-row w-full justify-start items-center bg-white px-[15px] py-[20px]",
                                                                    ),
                                                                    //FIXME: add search logic
                                                                    input {
                                                                        class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                                                                        r#type: "text",
                                                                        placeholder: "이름을 검색하세요.".to_string(),
                                                                        oninput: move |event| {
                                                                            event.stop_propagation();
                                                                            event.prevent_default();
                                                                        },
                                                                    }
                                                                }

                                                                for member in members.clone() {
                                                                    if !groups[index].member_list.iter().any(|m| m.id == member.member.user_id.clone()) {
                                                                        div { class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                                                            div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                                                                if member.member.name.clone().is_none() {
                                                                                    "{member.member.email}"
                                                                                } else {
                                                                                    {format!("{}", member.member.name.unwrap_or_default().clone())}
                                                                                }
                                                                            }
                                                                            div { class: "font-medium text-[#222222] text-[10px]",
                                                                                "{member.member.email}"
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
                                                                        // div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                                    //     Plus { width: "10", height: "10" }
                                    // }
                                    // div { class: "pl-[20px]",
                                    //     Expand { width: "18", height: "18" }
                                    // }
                                    }
                                    div { class: "p-4",
                                        div { class: "group relative",
                                            button {
                                                onclick: {
                                                    let group_id = groups[index].group_id.clone();
                                                    let group_name = groups[index].group_name.clone();
                                                    move |_| {
                                                        clicked_group_id.set(group_id.clone());
                                                        clicked_group_name.set(group_name.clone());
                                                    }
                                                },
                                                RowOption { width: 24, height: 24 }
                                            }
                                            nav {
                                                tabindex: "0",
                                                class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                ul { class: "py-1",
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| async move {
                                                            ctrl.open_remove_group_modal(props.lang, clicked_group_id, clicked_group_name)
                                                                .await;
                                                        },
                                                        "{translates.remove_group_li}"
                                                    }
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| async move {
                                                            ctrl.open_update_group_name_modal(
                                                                    props.lang,
                                                                    clicked_group_id,
                                                                    clicked_group_name,
                                                                )
                                                                .await;
                                                        },
                                                        "{translates.update_group_name_li}"
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
                    div { class: "ml-[12px] w-[24px] h-[24px]",
                        ArrowRight { width: "24", height: "24" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UpdateGroupNameModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    initialize_group_name: String,
    update_group_name: EventHandler<String>,
) -> Element {
    let i18n: UpdateGroupNameModalTranslate = translate(&lang);
    let mut group_name = use_signal(|| initialize_group_name);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                {i18n.update_group_name_info}
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.update_group_name_hint,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    {i18n.update_group_name_warning}
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {
                        update_group_name.call(group_name());
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.update} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

#[component]
pub fn RemoveGroupModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    remove_group: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveGroupModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start ",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_warning} }
                div { {i18n.remove_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: Event<MouseData>| {
                        remove_group.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.remove} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

#[component]
pub fn CreateGroupModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    oncreate: EventHandler<CreateGroupRequest>,
) -> Element {
    let i18n: CreateGroupModalTranslate = translate(&lang);
    let mut group_name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start ",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.input_contents,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    "{i18n.create_group_description}"
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    {i18n.add_team_member}
                }
                div { class: "flex flex-row w-full justify-center items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px]",
                    div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                        {i18n.team_member}
                    }
                    div { class: "flex flex-row w-full h-[45px] justify-center items-start bg-[#f7f7f7] rounded-md" }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]",
                    {i18n.invite_project}
                }
                div { class: "flex flex-col w-full justify-center items-start bg-white border border-[#bfc8d9] rounded-[8px] p-[24px]",
                    div { class: "flex flex-row w-full justify-center items-start mb-[10px]",
                        div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                            {i18n.public_opinion}
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-center items-start bg-[#f7f7f7] rounded-md" }
                    }
                    div { class: "flex flex-row w-full justify-center items-start",
                        div { class: "flex flex-row justify-start items-center text-[#222222] font-medium text-[15px] mr-[3px] w-[40px] h-[45px]",
                            {i18n.investigation}
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-center items-start bg-[#f7f7f7] rounded-md" }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                button {
                    class: "flex flex-row w-[110px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                    //FIXME: add members, projects content
                    onclick: move |_| async move {
                        oncreate
                            .call(CreateGroupRequest {
                                name: group_name(),
                                members: vec![],
                                public_opinion_projects: vec![],
                                investigation_projects: vec![],
                            });
                    },
                    Folder { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", {i18n.create} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}
