use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{
    AttributeItemResponse, AttributeResponse, CreateAttributeRequest, CreatePanelRequest,
    PanelResponse, UpdateAttributeRequest, UpdatePanelRequest,
};

use crate::{
    api::common::CommonQueryResponse,
    service::{attribute_api::AttributeApi, panel_api::PanelApi, popup_service::PopupService},
};

use super::{
    i18n::PanelTranslate,
    page::{
        RemoveAttributeModal, RemovePanelModal, UpdateAttributeNameModal, UpdatePanelNameModal,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    panels: Signal<Vec<PanelResponse>>,
    attributes: Signal<Vec<AttributeResponse>>,
    popup_service: Signal<PopupService>,
    translate: Signal<PanelTranslate>,

    attribute_resource:
        Resource<Result<CommonQueryResponse<models::prelude::AttributeResponse>, ServerFnError>>,
    panel_resource:
        Resource<Result<CommonQueryResponse<models::prelude::PanelResponse>, ServerFnError>>,

    attribute_bookmark: Signal<Option<String>>,
    panel_bookmark: Signal<Option<String>>,

    attribute_api: AttributeApi,
    panel_api: PanelApi,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let attribute_api: AttributeApi = use_context();
        let panel_api: PanelApi = use_context();
        let translate: PanelTranslate = translate(&lang);

        let mut attribute_bookmark = Signal::new(None);
        let mut panel_bookmark = Signal::new(None);

        let attribute_resource: Resource<
            Result<CommonQueryResponse<models::prelude::AttributeResponse>, ServerFnError>,
        > = use_resource(move || {
            let api = attribute_api.clone();
            let bookmark = attribute_bookmark();
            //FIXME: add bookmark
            async move {
                let res = api.list_attributes(Some(5), bookmark).await;

                match res.clone() {
                    Ok(v) => {
                        if let Some(bookmark) = v.bookmark {
                            if attribute_bookmark() != Some(bookmark.clone()) {
                                attribute_bookmark.set(Some(bookmark.clone()));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
                    }
                }

                res
            }
        });

        let panel_resource: Resource<
            Result<CommonQueryResponse<models::prelude::PanelResponse>, ServerFnError>,
        > = use_resource(move || {
            let api = panel_api.clone();
            //FIXME: add bookmark
            async move {
                let res = api.list_panels(Some(5), None).await;
                match res.clone() {
                    Ok(v) => {
                        if let Some(bookmark) = v.bookmark {
                            if panel_bookmark() != Some(bookmark.clone()) {
                                panel_bookmark.set(Some(bookmark.clone()));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
                    }
                }

                res
            }
        });

        let mut ctrl = Self {
            panels: use_signal(|| vec![]),
            attributes: use_signal(|| vec![]),
            popup_service: use_signal(|| popup_service),
            translate: use_signal(|| translate),

            attribute_resource,
            panel_resource,

            attribute_bookmark,
            panel_bookmark,

            attribute_api,
            panel_api,
        };

        match panel_resource.value()() {
            Some(panel) => {
                if panel.is_ok() {
                    ctrl.panels.set(panel.unwrap().items);
                }
            }
            _ => {}
        }

        match attribute_resource.value()() {
            Some(attribute) => {
                if attribute.is_ok() {
                    ctrl.attributes.set(attribute.unwrap().items);
                }
            }
            _ => {}
        }

        use_context_provider(|| ctrl);
        ctrl
    }

    pub async fn next_panel_clicked(&mut self) {
        self.panel_resource.restart();
    }

    pub fn get_panels(&self) -> Vec<PanelResponse> {
        (self.panels)()
    }

    pub fn get_attributes(&self) -> Vec<AttributeResponse> {
        (self.attributes)()
    }

    pub fn get_attribute_bookmark(&self) -> Option<String> {
        (self.attribute_bookmark)()
    }

    pub fn get_panel_bookmark(&self) -> Option<String> {
        (self.panel_bookmark)()
    }

    pub async fn create_attribute(&self, req: CreateAttributeRequest) {
        let api: AttributeApi = self.attribute_api;
        let mut attribute_resource = self.attribute_resource;

        let _ = api.create_attribute(req).await;
        attribute_resource.restart();
    }

    pub async fn create_panel(&self, req: CreatePanelRequest) {
        let api: PanelApi = self.panel_api;
        let mut panel_resource = self.panel_resource;

        let _ = api.create_panel(req).await;
        panel_resource.restart();
    }

    pub async fn open_remove_attribute(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let api: AttributeApi = self.attribute_api;
        let attributes = self.get_attributes();
        let attribute_id = attributes[index].id.clone();

        let mut attribute_resource = self.attribute_resource;

        popup_service
            .open(rsx! {
                RemoveAttributeModal {
                    lang,
                    remove_click: move |_e: MouseEvent| {
                        let attribute_id = attribute_id.clone();
                        async move {
                            tracing::debug!("remove attribute clicked: {index}");
                            let _ = api.remove_attribute(attribute_id).await;
                            attribute_resource.restart();
                            popup_service.close();
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_attribute")
            .with_title(translate.remove_attribute);
    }

    pub async fn open_update_attribute_name(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let api: AttributeApi = self.attribute_api;
        let attributes = self.get_attributes();
        let attribute = attributes[index].clone();

        let mut attribute_resource = self.attribute_resource;

        popup_service
            .open(rsx! {
                UpdateAttributeNameModal {
                    lang,
                    onupdate: move |name: String| {
                        let attribute_id = attribute.id.clone();
                        let name = name.clone();
                        let attribute_items = attribute.attribute.clone();
                        async move {
                            tracing::debug!("update attribute clicked: {index} {name}");
                            let _ = api
                                .update_attribute(
                                    attribute_id,
                                    UpdateAttributeRequest {
                                        name,
                                        attribute_items,
                                    },
                                )
                                .await;
                            attribute_resource.restart();
                            popup_service.close();
                        }
                    },
                    initial_value: attributes[index].name.clone().unwrap_or_default(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_attribute_name")
            .with_title(translate.update_attribute_name);
    }

    pub async fn open_remove_panel(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
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
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;

        popup_service
            .open(rsx! {
                UpdatePanelNameModal {
                    lang,
                    onupdate: move |name: String| {
                        let panel_id = panel.id.clone();
                        let name = name.clone();
                        let count = panel.count.unwrap_or(0);
                        let attribute = panel.attribute.clone();
                        async move {
                            tracing::debug!("update panel clicked: {index} {name}");
                            let _ = api
                                .update_panel(
                                    panel_id,
                                    UpdatePanelRequest {
                                        name,
                                        count,
                                        attribute,
                                    },
                                )
                                .await;
                            panel_resource.restart();
                            popup_service.close();
                        }
                    },
                    initial_value: panels[index].name.clone().unwrap_or_default(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_panel_name")
            .with_title(translate.update_panel_name);
    }

    pub async fn update_attribute(
        &self,
        index: usize,
        attribute_items: Vec<AttributeItemResponse>,
    ) {
        tracing::debug!("update attribute: {} {:?}", index, attribute_items);
        let api: AttributeApi = self.attribute_api;
        let attributes = self.get_attributes();
        let attr = attributes[index].clone();

        let mut attribute_resource = self.attribute_resource;

        let _ = api
            .update_attribute(
                attr.id.clone(),
                UpdateAttributeRequest {
                    name: attr.name.clone().unwrap_or_default(),
                    attribute_items,
                },
            )
            .await;

        attribute_resource.restart();
    }

    pub async fn update_panel_name(&self, index: usize, name: String) {
        tracing::debug!("update update_panel_name: {} {:?}", index, name);
        let api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;

        let _ = api
            .update_panel(
                panel.id.clone(),
                UpdatePanelRequest {
                    name,
                    count: panel.count.unwrap_or_default(),
                    attribute: panel.attribute,
                },
            )
            .await;

        panel_resource.restart();
    }

    pub async fn update_attribute_name(&self, index: usize, name: String) {
        tracing::debug!("update update_attribute_name: {} {:?}", index, name);
        let api: AttributeApi = self.attribute_api;
        let attributes = self.get_attributes();
        let attr = attributes[index].clone();

        let mut attribute_resource = self.attribute_resource;

        let _ = api
            .update_attribute(
                attr.id.clone(),
                UpdateAttributeRequest {
                    name,
                    attribute_items: attr.attribute,
                },
            )
            .await;

        attribute_resource.restart();
    }

    pub async fn update_panel_attribute(&self, index: usize, attribute: Vec<AttributeResponse>) {
        tracing::debug!("update panel attribute: {} {:?}", index, attribute);
        let api: PanelApi = self.panel_api;
        let panels = self.get_panels();
        let panel = panels[index].clone();

        let mut panel_resource = self.panel_resource;

        let panel_id = panel.id.clone();
        let name = panel.name.clone();
        let count = panel.count.unwrap_or(0);
        let attribute = attribute.clone();

        let _ = api
            .update_panel(
                panel_id,
                UpdatePanelRequest {
                    name: name.unwrap_or_default(),
                    count,
                    attribute,
                },
            )
            .await;
        panel_resource.restart();
    }
}
