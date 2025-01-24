use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{AttributeSummary, PanelSummary};

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

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    panels: Signal<Vec<PanelSummary>>,
    attributes: Signal<Vec<AttributeSummary>>,
    popup_service: Signal<PopupService>,
    translate: Signal<PanelTranslate>,

    attribute_resource:
        Resource<Result<CommonQueryResponse<models::prelude::AttributeSummary>, ServerFnError>>,
    panel_resource:
        Resource<Result<CommonQueryResponse<models::prelude::PanelSummary>, ServerFnError>>,

    attribute_bookmark: Signal<Option<String>>,
    panel_bookmark: Signal<Option<String>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let attribute_api: AttributeApi = use_context();
        let panel_api: PanelApi = use_context();
        let translate: PanelTranslate = translate(&lang);

        let mut attribute_bookmark = Signal::new(None);
        let mut panel_bookmark = Signal::new(None);

        let attribute_resource: Resource<
            Result<CommonQueryResponse<models::prelude::AttributeSummary>, ServerFnError>,
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
            Result<CommonQueryResponse<models::prelude::PanelSummary>, ServerFnError>,
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

    pub fn get_panels(&self) -> Vec<PanelSummary> {
        tracing::debug!("this line come panel: {:?}", (self.panels)());
        (self.panels)()
    }

    pub fn get_attributes(&self) -> Vec<AttributeSummary> {
        (self.attributes)()
    }

    pub fn open_remove_attribute(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        popup_service
            .open(rsx! {
                RemoveAttributeModal {
                    lang,
                    remove_click: move |_| {
                        tracing::debug!("remove attribute clicked: {index}");
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_attribute")
            .with_title(translate.remove_attribute);
    }

    pub fn open_update_attribute_name(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let attributes = self.get_attributes();
        popup_service
            .open(rsx! {
                UpdateAttributeNameModal {
                    lang,
                    onupdate: move |name: String| {
                        tracing::debug!("update attribute clicked: {index} {name}");
                    },
                    initial_value: attributes[index].name.clone(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_attribute_name")
            .with_title(translate.update_attribute_name);
    }

    pub fn open_remove_panel(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        popup_service
            .open(rsx! {
                RemovePanelModal {
                    lang,
                    remove_click: move |_| {
                        tracing::debug!("remove panel clicked: {index}");
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_panel")
            .with_title(translate.remove_panel);
    }

    pub fn open_update_panel_name(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let panels = self.get_panels();
        popup_service
            .open(rsx! {
                UpdatePanelNameModal {
                    lang,
                    onupdate: move |name: String| {
                        tracing::debug!("update panel clicked: {index} {name}");
                    },
                    initial_value: panels[index].name.clone(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_panel_name")
            .with_title(translate.update_panel_name);
    }
}
