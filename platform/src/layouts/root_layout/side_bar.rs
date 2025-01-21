#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    components::icons::{BottomArrow, Logo},
    routes::Route,
    service::organization_api::OrganizationApi,
};

use super::Language;

#[derive(PartialEq, Props, Clone)]
pub struct SidebarProps {
    onselected: EventHandler<SelectedMenu>,
    selected_menu: String,
    lang: Language,
}

#[component]
pub fn SideBar(props: SidebarProps) -> Element {
    let mut api: OrganizationApi = use_context();

    let _ = use_resource(move || {
        let mut api = api.clone();
        async move {
            let organizations = api.list_organizations(Some(100), None).await;
            let items = organizations.unwrap_or_default().items;
            api.set_organization(items);
        }
    });

    let organizations = api.get_organizations();
    let selected_organization = api.get_selected_organization_id();

    tracing::debug!("selected organization: {selected_organization}");

    let organization_menus: Vec<MenuItem> = organizations
        .iter()
        .map(|v| MenuItem {
            id: v.organization_id.clone(),
            title: v.organization_name.clone(),
            is_selected: false,
            link: None,
        })
        .collect();
    rsx! {
        div { class: "flex-col min-w-[250px] grow bg-[#435393] justify-between items-between rounded-tr-lg rounded-br-lg",
            div { class: "flex flex-col w-full h-full justify-between items-between",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-start items-center mt-[33px] ml-[18px] mb-[40px]",
                        Logo { width: "31", height: "31" }
                        div { class: "font-bold text-[#b8d5fa] text-[18px] pl-[5px]",
                            "VOICE KOREA"
                        }
                    }
                    div { class: "flex flex-col grow items-center w-full",
                        SectionMenus {
                            onselected: move |menu: SelectedMenu| {
                                api.set_selected_organization_id(menu.id.clone());
                            },
                            title: "{Organization}".to_string(),
                            menus: organization_menus,
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "Overview".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "프로젝트 검색".to_string(),
                                    is_selected: props.selected_menu == "프로젝트 검색",
                                    link: Some(Route::DashboardPage {
                                        lang: props.lang,
                                    }),
                                },
                                MenuItem {
                                    id: "".to_string(),
                                    title: "기본 프로젝트 가져오기".to_string(),
                                    is_selected: props.selected_menu == "기본 프로젝트 가져오기",
                                    link: Some(Route::DashboardPage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "조직 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "팀원 관리".to_string(),
                                    is_selected: props.selected_menu == "팀원 관리",
                                    link: Some(Route::MemberPage {
                                        lang: props.lang,
                                    }),
                                },
                                MenuItem {
                                    id: "".to_string(),
                                    title: "그룹 관리".to_string(),
                                    is_selected: props.selected_menu == "그룹 관리",
                                    link: Some(Route::GroupPage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "조사 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "조사 관리".to_string(),
                                    is_selected: props.selected_menu == "조사 관리",
                                    link: Some(Route::SurveyPage {
                                        lang: props.lang,
                                    }),
                                },
                                MenuItem {
                                    id: "".to_string(),
                                    title: "질문 뱅크".to_string(),
                                    is_selected: props.selected_menu == "질문 뱅크",
                                    link: Some(Route::DashboardPage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "공론 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "공론 관리".to_string(),
                                    is_selected: props.selected_menu == "공론 관리",
                                    link: Some(Route::OpinionPage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "속성 & 패널 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "속성 & 패널 관리".to_string(),
                                    is_selected: props.selected_menu == "속성 & 패널 관리",
                                    link: Some(Route::PanelPage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "자료 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    id: "".to_string(),
                                    title: "자료 관리".to_string(),
                                    is_selected: props.selected_menu == "자료 관리",
                                    link: Some(Route::ResourcePage {
                                        lang: props.lang,
                                    }),
                                },
                            ],
                        }
                    }
                }
                div { class: "flex flex-row w-full h-full justify-end items-end p-[10px]",
                    div { class: "flex flex-row w-[110px]",
                        div { class: "text-white text-[14px] font-normal pr-[10px]",
                            {"사용자 설정"}
                        }
                        img {
                            src: asset!("/public/images/config.png"),
                            class: "w-[24px] h-[24px]",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectedMenu {
    pub category: String,
    pub menu: String,
    pub id: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct MenuItem {
    id: String,
    title: String,
    is_selected: bool,
    link: Option<Route>,
}

#[component]
pub fn SectionMenus(
    onselected: EventHandler<SelectedMenu>,
    title: String,
    menus: Vec<MenuItem>,
) -> Element {
    let mut expanded = use_signal(|| true);

    rsx! {
        div { class: "flex flex-col w-full px-[18px] mb-[40px]",
            div {
                class: "flex flex-row h-[45px] w-full items-center justify-between cursor-pointer",
                onclick: move |_| {
                    expanded.set(!expanded());
                },
                div { class: "font-semibold text-[14px] text-white", "{title}" }
                if expanded() {
                    BottomArrow { width: "18".to_string(), height: "14".to_string() }
                } else {
                    div { style: "transform: scaleY(-1);",
                        BottomArrow { width: "18".to_string(), height: "14".to_string() }
                    }
                }
            }
            if expanded() {
                for menu in menus {
                    {
                        let title = title.clone();
                        let cm = menu.clone();
                        rsx! {
                            if menu.link.is_some() {
                                Link {
                                    onclick: move |_evt| {
                                        onselected
                                            .call(SelectedMenu {
                                                id: cm.id.clone(),
                                                category: title.clone(),
                                                menu: cm.title.clone(),
                                            });
                                    },
                                    to: menu.link.unwrap(),
                                    class: format!(
                                        "flex flex-row h-[45px] w-full px-[16px] items-center justify-start {}",
                                        if menu.is_selected { "rounded-lg bg-[#182248]" } else { "" },
                                    ),
                                    div { class: "flex font-medium text-[14px] text-[#daeaff]", "{menu.title}" }
                                }
                            } else {
                                div {
                                    onclick: move |_evt| {
                                        onselected
                                            .call(SelectedMenu {
                                                id: cm.id.clone(),
                                                category: title.clone(),
                                                menu: cm.title.clone(),
                                            });
                                    },
                                    class: format!(
                                        "flex flex-row h-[45px] w-full px-[16px] items-center justify-start cursor-pointer {}",
                                        if menu.is_selected { "rounded-lg bg-[#182248]" } else { "" },
                                    ),
                                    div { class: "flex font-medium text-[14px] text-[#daeaff]", "{menu.title}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
