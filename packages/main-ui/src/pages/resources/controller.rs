#![allow(unused)]

use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    AccessLevel, Field, QueryResponse, ResourceCreateRequest, ResourceGetResponse, ResourceQuery,
    ResourceSummary, ResourceType, ResourceUpdateRequest, Source, UsagePurpose,
};

use crate::{
    api, config,
    pages::resources::components::create_resource_modal::{
        CreateResourceModal, File, ModifyResourceModal, RemoveResourceModal,
    },
    service::{login_service::LoginService, popup_service::PopupService},
};
use dioxus_translate::translate;

use super::components::create_resource_modal::i18n::CreateResourceModalTranslate;
use super::i18n::ResourceTranslate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateResource {
    ResourceType(Option<ResourceType>),
    Field(Option<Field>),
    UsagePurpose(Option<UsagePurpose>),
    Source(Option<Source>),
    AccessLevel(Option<AccessLevel>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    ResourceType,
    Field,
    UsagePurpose,
    Source,
    AccessLevel,
    Title,
    LinkedDeliberationSurvey,
    LastModifiedDate,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    user: LoginService,
    popup_service: PopupService,
    sort_order: Signal<Option<(SortOrder, OrderBy)>>,
    editing_row_index: Signal<i32>,
    total_count: i64,
    page: Signal<usize>,
    resources: Signal<Vec<ResourceSummary>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Result<Self, RenderError> {
        let user: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 20;

        //FIXME:
        let mut resources = use_signal(Vec::new);
        let mut total_count = 0;
        let _ = use_resource(move || {
            let page = page();
            async move {
                match models::Resource::get_client(&config::get().api_url)
                    .query(models::ResourceQuery::new(size).with_page(page))
                    .await
                {
                    Ok(v) => {
                        resources.set(v.items);
                        total_count = v.total_count;
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                };
            }
        });
        let ctrl = Self {
            lang,
            user: use_context(),
            popup_service: use_context(),
            sort_order: use_signal(|| None),
            editing_row_index: use_signal(|| -1),
            page,
            total_count,
            resources,
        };
        Ok(ctrl)
    }
    pub fn change_page(&mut self, page: usize) {
        self.page.set(page);
    }
    pub fn get_resources(&self) -> Vec<ResourceSummary> {
        (self.resources)().clone()
    }

    pub fn is_sorted_by(&self, order_by: OrderBy) -> Option<SortOrder> {
        match (self.sort_order)() {
            Some((order, cur_order_by)) => {
                if order_by == cur_order_by {
                    Some(order)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn handle_sorting_order(&mut self, order_by: OrderBy) {
        if let Some((prev_order, prev_order_by)) = (self.sort_order)() {
            if order_by == prev_order_by {
                if prev_order == SortOrder::Asc {
                    self.sort_order.set(Some((SortOrder::Desc, order_by)));
                } else {
                    self.sort_order.set(None);
                }
            } else {
                self.sort_order.set(Some((SortOrder::Asc, order_by)));
            }
        } else {
            self.sort_order.set(Some((SortOrder::Asc, order_by)));
        }
    }

    pub fn is_editing(&self, index: i32) -> bool {
        // (self.editing_row_index)().is_some_and(|editing_index| editing_index == index)
        (self.editing_row_index)() == index
    }

    pub fn handle_change_editing_row(&mut self, next_index: i32) {
        self.editing_row_index.set(next_index);
    }
    pub fn handle_update_resource(&mut self, index: usize, field: UpdateResource) {
        let mut resources = self.resources.write();
        match field {
            UpdateResource::ResourceType(v) => resources[index].resource_type = v,
            UpdateResource::Field(v) => resources[index].field = v,
            UpdateResource::UsagePurpose(v) => resources[index].usage_purpose = v,
            UpdateResource::Source(v) => resources[index].source = v,
            UpdateResource::AccessLevel(v) => resources[index].access_level = v,
        }
    }

    pub async fn update(&self, index: usize) {
        let resource = self.resources.read()[index].clone();
        // TODO: Update Resource
    }
    pub fn convert_timestamp_to_date(timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub async fn create_resource(
        &self,
        title: String,
        resource_type: Option<ResourceType>,
        field: Option<Field>,
        usage_purpose: Option<UsagePurpose>,
        source: Option<Source>,
        access_level: Option<AccessLevel>,
    ) -> Result<(), models::ApiError> {
        let org = self.user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::Resource::get_client(&config::get().api_url);

        match client
            .create(
                title,
                resource_type,
                field,
                usage_purpose,
                source,
                access_level,
                org_id,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub async fn update_resource(
        &self,
        index: usize,
        title: String,
    ) -> Result<(), models::ApiError> {
        let client = models::Resource::get_client(&config::get().api_url);
        let req = self.resources.read()[index].clone();
        match client
            .update(
                &req.id,
                title,
                req.resource_type,
                req.field,
                req.usage_purpose,
                req.source,
                req.access_level,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Resource Update Failed: {:?}", e);
                Err(models::ApiError::ApiCallError(e.to_string()))
            }
        }
    }

    pub async fn remove_resource(&self, id: String) -> Result<(), models::ApiError> {
        //TODO: remove resource
        Err(models::ApiError::InvalidAction)
    }
}

impl Controller {
    pub fn open_create_resource_modal(&self) {
        let mut popup_service = self.popup_service.clone();
        let translate: CreateResourceModalTranslate = translate(&self.lang);
        let lang = self.lang;
        let ctrl = self.clone();
        popup_service
            .open(rsx! {
                CreateResourceModal {
                    lang,
                    onupload: move |(title, resource_type, field, purpose, source, access_level, _files)| {
                        async move {
                            ctrl.create_resource(
                                    title,
                                    resource_type,
                                    field,
                                    purpose,
                                    source,
                                    access_level,
                                )
                                .await;
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create resource")
            .with_title(translate.title);
    }

    pub fn open_modify_resource_modal(&self, index: usize) {
        let resource = self.resources.read()[index].clone();
        let mut popup_service = self.popup_service.clone();
        let translate: ResourceTranslate = translate(&self.lang);
        let lang = self.lang;
        let ctrl = self.clone();
        popup_service
            .open(rsx! {
                ModifyResourceModal {
                    lang,
                    title: resource.title,
                    files: vec![],
                    onupload: move |(title, files): (String, Vec<File>)| {
                        async move {
                            ctrl.update_resource(index, title).await;
                        }
                    },
                    onclose: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("modify resource")
            .with_title(translate.update_resource_li);
    }
}
// impl Controller {
//     pub fn new(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
//         let client = Resource::get_client(&crate::config::get().api_url);

//         // let metadata_resource: Resource<
//         //     Result<CommonQueryResponse<ResourceMetadata>, ServerFnError>,
//         // > = use_resource(move || {
//         //     let api = resource_api.clone();
//         //     async move {
//         //         let res = api.list_metadata(Some(100), None).await;
//         //         res
//         //     }
//         // });

//         let mut ctrl = Self {
//             resources: use_signal(|| vec![]),
//             popup_service,

//             resource_api,
//             metadata_resource,
//         };

//         match metadata_resource.value()() {
//             Some(resource) => {
//                 if resource.is_ok() {
//                     ctrl.resources.set(resource.unwrap().items);
//                 }
//             }
//             _ => {}
//         }

//         ctrl
//     }

//     pub fn open_create_material(&self, lang: Language) {
//         let mut popup_service = (self.popup_service)().clone();
//         let translate = (self.translate)().clone();

//         let total_types = self.get_total_types();
//         let total_fields = self.get_total_fields();
//         let total_purposes = self.get_total_purposes();
//         let total_resources = self.get_total_resources();
//         let total_authorities = self.get_total_authorities();

//         let api: ResourceApi = self.resource_api;
//         let mut metadata_resource = self.metadata_resource;

//         popup_service
//             .open(rsx! {
//                 CreateMaterialModal {
//                     lang,
//                     onupload: move |req: Create| async move {
//                         tracing::debug!("upload material clicked");
//                         let _ = api.create_metadata(req).await;
//                         metadata_resource.restart();
//                         popup_service.close();
//                     },
//                     onclose: move |_| {
//                         popup_service.close();
//                     },

//                     total_types,
//                     total_fields,
//                     total_purposes,
//                     total_resources,
//                     total_authorities,
//                 }
//             })
//             .with_id("create material")
//             .with_title(translate.upload_material);
//     }

//     pub fn open_update_material(&self, lang: Language, index: usize) {
//         let mut popup_service = (self.popup_service)().clone();
//         let translate = (self.translate)().clone();
//         let api: ResourceApi = self.resource_api;

//         let materials = self.get_resources();
//         let material = materials[index].clone();

//         let mut metadata_resource = self.metadata_resource;
//         popup_service
//             .open(rsx! {
//                 UpdateMaterialModal {
//                     lang,
//                     onupload: move |files: Vec<File>| {
//                         let id = material.id.clone();
//                         ResourceUpdateRequest {
//                             title: material.title.clone(),
//                             resource_type: material
//                         }
//                         async move {
//                             tracing::debug!("update material clicked: {index}");
//                             let _ = api.update_metadata(id, req).await;
//                             metadata_resource.restart();
//                             popup_service.close();
//                         }
//                     },
//                     initial_title: material.name.clone(),
//                     onclose: move |_| {
//                         popup_service.close();
//                     },
//                 }
//             })
//             .with_id("update_material")
//             .with_title(translate.update_material_li);
//     }

//     pub async fn update_metadata(&self, index: usize, req: UpdateMetadataRequest) {
//         tracing::debug!("update metadata: {index} {:?}", req);
//         let api: ResourceApi = self.resource_api;

//         let materials = self.get_resources();
//         let material = materials[index].clone();
//         let mut metadata_resource = self.metadata_resource;

//         let _ = api.update_metadata(material.id.clone(), req).await;
//         metadata_resource.restart();
//     }
//     pub async fn upload_files(&self, files: Vec<File>) {
//         tracing::debug!("upload files: {:?}", files);
//     }
//     pub fn open_remove_material(&self, lang: Language, index: usize) {
//         let mut popup_service = (self.popup_service)().clone();
//         let translate = (self.translate)().clone();
//         let api: ResourceApi = self.resource_api;
//         let resources = self.get_resources();
//         let resource_id = resources[index].id.clone();

//         let mut metadata_resource = self.metadata_resource;

//         popup_service
//             .open(rsx! {
//                 RemoveMaterialModal {
//                     lang,
//                     onremove: move |_e: MouseEvent| {
//                         let resource_id = resource_id.clone();
//                         async move {
//                             tracing::debug!("remove resource clicked: {index}");
//                             let _ = api.remove_metadata(resource_id).await;
//                             metadata_resource.restart();
//                             popup_service.close();
//                         }
//                     },
//                     onclose: move |_| {
//                         popup_service.close();
//                     },
//                 }
//             })
//             .with_id("remove_material")
//             .with_title(translate.remove_material);
//     }
// }
