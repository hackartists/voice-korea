use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::prelude::{
    CreateMetadataRequest, Field, MetadataAuthority, MetadataPurpose, MetadataSource,
    MetadataSummary, MetadataType, UpdateMetadataRequest,
};

use crate::{
    api::common::CommonQueryResponse,
    service::{metadata_api::ResourceApi, popup_service::PopupService},
};
use dioxus_translate::translate;

use super::{
    i18n::ResourceTranslate,
    page::{CreateMaterialModal, RemoveMaterialModal, UpdateMaterialModal},
};

#[derive(Debug, Clone, PartialEq)]
pub struct SelectMetadataType {
    pub metadata_type: Option<MetadataType>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectMetadataField {
    pub metadata_field: Option<Field>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectMetadataPurpose {
    pub metadata_purpose: Option<MetadataPurpose>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectMetadataSource {
    pub metadata_source: Option<MetadataSource>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectMetadataAuthority {
    pub metadata_authority: Option<MetadataAuthority>,
    pub name: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    popup_service: Signal<PopupService>,
    translate: Signal<ResourceTranslate>,
    resources: Signal<Vec<MetadataSummary>>,

    total_types: Signal<Vec<String>>,
    total_fields: Signal<Vec<String>>,
    total_purposes: Signal<Vec<String>>,
    total_resources: Signal<Vec<String>>,
    total_authorities: Signal<Vec<String>>,

    resource_api: ResourceApi,
    metadata_resource: Resource<Result<CommonQueryResponse<MetadataSummary>, ServerFnError>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let resource_api: ResourceApi = use_context();

        let translate: ResourceTranslate = translate(&lang);
        let total_authorities = vec![
            translate.public_material.to_string(),
            translate.restricted_material.to_string(),
            translate.private_material.to_string(),
        ];

        let total_resources = vec![
            translate.internal_material.to_string(),
            translate.external_material.to_string(),
            translate.agency.to_string(),
            translate.private_enterprise.to_string(),
        ];

        let total_purposes = vec![
            translate.policy_development.to_string(),
            translate.academic_research.to_string(),
            translate.public_discussion_document.to_string(),
            translate.education_document.to_string(),
        ];
        let total_fields = vec![
            translate.economy.to_string(),
            translate.society.to_string(),
            translate.environment.to_string(),
            translate.education.to_string(),
            translate.culture.to_string(),
            translate.labor.to_string(),
            translate.city.to_string(),
            translate.technology.to_string(),
            translate.health.to_string(),
            translate.politic.to_string(),
        ];

        let total_types = vec![
            translate.report.to_string(),
            translate.statistics.to_string(),
            translate.survey.to_string(),
            translate.thesis.to_string(),
            translate.presentations.to_string(),
            translate.media.to_string(),
        ];

        let metadata_resource: Resource<
            Result<CommonQueryResponse<MetadataSummary>, ServerFnError>,
        > = use_resource(move || {
            let api = resource_api.clone();
            async move {
                let res = api.list_metadata(Some(100), None).await;
                res
            }
        });

        let mut ctrl = Self {
            resources: use_signal(|| vec![]),
            popup_service: use_signal(|| popup_service),
            translate: use_signal(|| translate),

            total_types: use_signal(|| total_types),
            total_fields: use_signal(|| total_fields),
            total_purposes: use_signal(|| total_purposes),
            total_resources: use_signal(|| total_resources),
            total_authorities: use_signal(|| total_authorities),

            resource_api,
            metadata_resource,
        };

        match metadata_resource.value()() {
            Some(resource) => {
                if resource.is_ok() {
                    ctrl.resources.set(resource.unwrap().items);
                }
            }
            _ => {}
        }

        ctrl
    }

    pub fn get_total_types(&self) -> Vec<String> {
        (self.total_types)()
    }

    pub fn get_total_fields(&self) -> Vec<String> {
        (self.total_fields)()
    }

    pub fn get_total_purposes(&self) -> Vec<String> {
        (self.total_purposes)()
    }

    pub fn get_total_resources(&self) -> Vec<String> {
        (self.total_resources)()
    }

    pub fn get_total_authorities(&self) -> Vec<String> {
        (self.total_authorities)()
    }

    pub fn get_resources(&self) -> Vec<MetadataSummary> {
        (self.resources)()
    }

    pub fn translate_metadata_type(
        &self,
        lang: Language,
        metadata_type: MetadataType,
    ) -> &'static str {
        metadata_type.translate(&lang)
    }

    pub fn metadata_type_from_str(&self, metadata_type: String) -> Option<MetadataType> {
        let metadata_type = metadata_type.parse::<MetadataType>();

        match metadata_type {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn translate_metadata_field(&self, lang: Language, metadata_field: Field) -> &'static str {
        metadata_field.translate(&lang)
    }

    pub fn metadata_field_from_str(&self, metadata_field: String) -> Option<Field> {
        let metadata_field = metadata_field.parse::<Field>();

        match metadata_field {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn translate_metadata_purpose(
        &self,
        lang: Language,
        metadata_purpose: MetadataPurpose,
    ) -> &'static str {
        metadata_purpose.translate(&lang)
    }

    pub fn metadata_purpose_from_str(&self, metadata_purpose: String) -> Option<MetadataPurpose> {
        let metadata_purpose = metadata_purpose.parse::<MetadataPurpose>();

        match metadata_purpose {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn translate_metadata_source(
        &self,
        lang: Language,
        metadata_source: MetadataSource,
    ) -> &'static str {
        metadata_source.translate(&lang)
    }

    pub fn metadata_source_from_str(&self, metadata_source: String) -> Option<MetadataSource> {
        let metadata_source = metadata_source.parse::<MetadataSource>();

        match metadata_source {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn translate_metadata_authority(
        &self,
        lang: Language,
        metadata_authority: MetadataAuthority,
    ) -> &'static str {
        metadata_authority.translate(&lang)
    }

    pub fn metadata_authority_from_str(
        &self,
        metadata_authority: String,
    ) -> Option<MetadataAuthority> {
        let metadata_authority = metadata_authority.parse::<MetadataAuthority>();

        match metadata_authority {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn convert_timestamp_to_date(&self, timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn open_create_material(&self, lang: Language) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();

        let total_types = self.get_total_types();
        let total_fields = self.get_total_fields();
        let total_purposes = self.get_total_purposes();
        let total_resources = self.get_total_resources();
        let total_authorities = self.get_total_authorities();

        let api: ResourceApi = self.resource_api;
        let mut metadata_resource = self.metadata_resource;

        popup_service
            .open(rsx! {
                CreateMaterialModal {
                    lang,
                    onupload: move |req: CreateMetadataRequest| async move {
                        tracing::debug!("upload material clicked");
                        let _ = api.create_metadata(req).await;
                        metadata_resource.restart();
                        popup_service.close();
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },

                    total_types,
                    total_fields,
                    total_purposes,
                    total_resources,
                    total_authorities,
                }
            })
            .with_id("create material")
            .with_title(translate.upload_material);
    }

    pub fn open_update_material(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let api: ResourceApi = self.resource_api;

        let materials = self.get_resources();
        let material = materials[index].clone();

        let mut metadata_resource = self.metadata_resource;
        popup_service
            .open(rsx! {
                UpdateMaterialModal {
                    lang,
                    onupload: move |(name, urls): (String, Vec<String>)| {
                        let id = material.id.clone();
                        let req = UpdateMetadataRequest {
                            name,
                            urls,
                            metadata_type: material.metadata_type.clone(),
                            metadata_field: material.metadata_field.clone(),
                            metadata_purpose: material.metadata_purpose.clone(),
                            metadata_source: material.metadata_source.clone(),
                            metadata_authority: material.metadata_authority.clone(),
                            public_opinion_projects: material.public_opinion_projects.clone(),
                            public_survey_projects: material.public_survey_projects.clone(),
                        };
                        async move {
                            tracing::debug!("update material clicked: {index}");
                            let _ = api.update_metadata(id, req).await;
                            metadata_resource.restart();
                            popup_service.close();
                        }
                    },
                    initial_title: material.name.clone(),
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("update_material")
            .with_title(translate.update_material_li);
    }

    pub fn open_remove_material(&self, lang: Language, index: usize) {
        let mut popup_service = (self.popup_service)().clone();
        let translate = (self.translate)().clone();
        let api: ResourceApi = self.resource_api;
        let resources = self.get_resources();
        let resource_id = resources[index].id.clone();

        let mut metadata_resource = self.metadata_resource;

        popup_service
            .open(rsx! {
                RemoveMaterialModal {
                    lang,
                    onremove: move |_e: MouseEvent| {
                        let resource_id = resource_id.clone();
                        async move {
                            tracing::debug!("remove resource clicked: {index}");
                            let _ = api.remove_metadata(resource_id).await;
                            metadata_resource.restart();
                            popup_service.close();
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_material")
            .with_title(translate.remove_material);
    }
}
