#![allow(non_snake_case)]
use crate::{components::icons::Logout, prelude::*, service::popup_service::PopupZone};
use dioxus::prelude::*;
use side_bar::{SelectedMenu, SideBar};

pub mod header;
pub mod i18n;
pub mod side_bar;

use dioxus_translate::Language;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let mut selected_menu = use_signal(|| "".to_string());
    use dioxus_logger::tracing;

    let route: Route = use_route();
    let current_path = format!("{route}");

    use_effect({
        let current_selected = selected_menu();
        let current_path = current_path.clone();
        move || {
            if current_selected == "" {
                let new_menu = match current_path.as_str() {
                    path if path.contains("/group") => "그룹 관리".to_string(),
                    path if path.contains("/member") => "팀원 관리".to_string(),
                    path if path.contains("/opinions") => "공론 조사".to_string(),
                    path if path.contains("/surveys") => "여론 조사".to_string(),
                    path if path.contains("/panels") => "속성 & 패널 관리".to_string(),
                    path if path.contains("/resources") => "자료 관리".to_string(),
                    _ => "프로젝트 검색".to_string(),
                };

                if current_selected != new_menu {
                    selected_menu.set(new_menu);
                }
            }
        }
    });

    rsx! {
        div { class: "flex flex-col w-screen min-h-screen bg-white text-black",
            // Header {
            //     logout: translates.logout,
            //     lang,
            // }
            PopupZone {}
            div { class: "flex flex-row min-w-full max-w-full grow",
                SideBar {
                    onselected: move |selected: SelectedMenu| {
                        tracing::info!("selected menu {selected:?}");
                        selected_menu.set(selected.menu);
                    },
                    selected_menu: (selected_menu)(),
                    lang,
                }
                div { class: "flex flex-col grow w-full bg-[#f0f2fc] px-[60px] pt-[25px]",
                    div { class: "flex flex-row w-full justify-end items-end gap-[5px]",
                        Link {
                            class: "flex flex-row w-min justify-start items-start",
                            to: Route::LoginPage { lang },
                            div { class: "w-[20px] h-[20px]",
                                Logout { width: "20", height: "20" }
                            }
                            div { class: "ml-[5px] font-bold text-[#555462] text-[15px]",
                                "logout"
                            }
                        }
                    }
                    Outlet::<Route> {}
                }
            }
        }
    }
}
