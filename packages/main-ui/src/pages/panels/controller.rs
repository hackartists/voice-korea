use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    panel_v2::{PanelV2, PanelV2Summary},
    prelude::CreatePanelRequest,
    AttributeResponse,
};

use crate::service::{panel_api::PanelApi, popup_service::PopupService};

use super::{
    i18n::PanelTranslate,
    page::{RemovePanelModal, UpdatePanelNameModal},
};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    panels: Signal<Vec<PanelV2Summary>>,
    popup_service: PopupService,
    translate: Signal<PanelTranslate>,

    panel_resource: Resource<Vec<PanelV2Summary>>,
    panel_bookmark: Signal<Option<String>>,
    attributes: Signal<Vec<AttributeInfo>>,
    panel_api: PanelApi,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let panel_api: PanelApi = use_context();
        let translate: PanelTranslate = translate(&lang);
        let panel_bookmark = Signal::new(None);

        let panel_resource = use_resource(move || {
            //FIXME: add bookmark and fix to real organization id
            async move {
                match PanelV2::get_client(&crate::config::get().api_url)
                    .list_panels(100, None, "test".to_string())
                    .await
                {
                    Ok(d) => d.items,
                    Err(e) => {
                        tracing::error!("list panels failed: {e}");
                        vec![]
                    }
                }
            }
        });

        let trans = translate.clone();

        let mut ctrl = Self {
            panels: use_signal(|| vec![]),
            popup_service,
            translate: use_signal(|| trans),

            attributes: use_signal(|| vec![]),
            panel_resource,
            panel_bookmark,
            panel_api,
        };

        if ctrl.attributes.len() == 0 {
            ctrl.attributes.push(AttributeInfo {
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
            });

            ctrl.attributes.push(AttributeInfo {
                name: translate.clone().gender.to_string(),
                values: vec![
                    translate.clone().male.to_string(),
                    translate.clone().female.to_string(),
                ],
            });

            ctrl.attributes.push(AttributeInfo {
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
            });

            ctrl.attributes.push(AttributeInfo {
                name: translate.clone().salary.to_string(),
                values: vec![
                    translate.clone().tier_one.to_string(),
                    translate.clone().tier_two.to_string(),
                    translate.clone().tier_three.to_string(),
                    translate.clone().tier_four.to_string(),
                    translate.clone().tier_five.to_string(),
                ],
            });
        }

        match panel_resource.value()() {
            Some(panel) => {
                ctrl.panels.set(panel);
            }
            _ => {}
        }

        use_context_provider(|| ctrl);
        ctrl
    }

    pub async fn next_panel_clicked(&mut self) {
        self.panel_resource.restart();
    }

    pub fn get_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn get_attributes(&self) -> Vec<AttributeInfo> {
        (self.attributes)()
    }

    pub fn get_panel_bookmark(&self) -> Option<String> {
        (self.panel_bookmark)()
    }

    pub async fn create_panel(&self, req: CreatePanelRequest) {
        let api: PanelApi = self.panel_api;
        let mut panel_resource = self.panel_resource;

        let _ = api.create_panel(req).await;
        panel_resource.restart();
    }

    pub async fn open_setting_age_modal(&self, _lang: Language, index: usize) {
        let mut popup_service = self.popup_service.clone();
        let _translate = (self.translate)().clone();
        let panels = self.get_panels();
        let _panel_id = panels[index].id.clone();

        popup_service
            .open(rsx! {})
            .with_id("setting_age")
            .with_title("연령 속성 설정");
    }

    pub async fn open_remove_panel(&self, lang: Language, index: usize) {
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let panel_id = panels[index].id.clone();

        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                RemovePanelModal {
                    lang,
                    remove_click: move |_e: MouseEvent| {
                        let panel_id = panel_id.clone();
                        async move {
                            tracing::debug!("remove panel clicked: {index}");
                            let _ = api.remove_panel(panel_id).await;
                            panel_resource.restart();
                            popup_service.close();
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_panel")
            .with_title(translate.remove_panel);
    }

    pub async fn open_update_panel_name(&self, lang: Language, index: usize) {
        let mut popup_service = self.popup_service.clone();
        let translate = (self.translate)().clone();
        let _api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let _panel = panels[index].clone();

        let _panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                UpdatePanelNameModal {
                    lang,
                    // FIXME: implement panel name update logic
                    onupdate: move |_name: String| {},
                    initial_value: panels[index].name.clone(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_panel_name")
            .with_title(translate.update_panel_name);
    }

    pub async fn update_panel_name(&self, index: usize, name: String) {
        tracing::debug!("update update_panel_name: {} {:?}", index, name);
        let _api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let _panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;

        //TODO: implement update panel name logic

        // let _ = api
        //     .update_panel(
        //         panel.id.clone(),
        //         UpdatePanelRequest {
        //             name,
        //             count: panel.user_count as i64,
        //             attribute: panel.attribute,
        //         },
        //     )
        //     .await;

        panel_resource.restart();
    }

    pub async fn update_panel_attribute(&self, index: usize, attribute: Vec<AttributeResponse>) {
        tracing::debug!("update panel attribute: {} {:?}", index, attribute);
        let _api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let _panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;

        //FIXME: implement panel attribute logic
        // let panel_id = panel.id.clone();
        // let name = panel.name.clone();
        // let count = panel.count.unwrap_or(0);
        // let attribute = attribute.clone();

        // let _ = api
        //     .update_panel(
        //         panel_id,
        //         UpdatePanelRequest {
        //             name: name.unwrap_or_default(),
        //             count,
        //             attribute,
        //         },
        //     )
        //     .await;
        panel_resource.restart();
    }
}
