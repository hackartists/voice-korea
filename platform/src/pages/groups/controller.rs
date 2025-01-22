use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::service::group_api::GroupApi;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupSummary {
    pub group_id: String,
    pub group_name: String,
    pub member_count: u64,
    pub member_list: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub groups: Signal<Vec<GroupSummary>>,
    pub group_resource: Resource<
        Result<
            crate::api::common::CommonQueryResponse<models::prelude::GroupResponse>,
            ServerFnError,
        >,
    >,
}

impl Controller {
    pub fn init(_lang: Language) -> Self {
        let api: GroupApi = use_context();
        let group_resource: Resource<
            Result<
                crate::api::common::CommonQueryResponse<models::prelude::GroupResponse>,
                ServerFnError,
            >,
        > = use_resource(move || {
            let api = api.clone();
            async move { api.list_groups(Some(100), None).await }
        });
        let mut ctrl = Self {
            groups: use_signal(|| vec![]),
            group_resource,
        };

        let groups = if let Some(v) = group_resource.value()() {
            match v {
                Ok(d) => d
                    .items
                    .iter()
                    .map(|group| GroupSummary {
                        group_id: group.id.clone(),
                        group_name: group.name.clone(),
                        member_count: group.members.len() as u64,
                        member_list: group.members.iter().map(|v| v.user_name.clone()).collect(), // FIXME: fix to real member list
                    })
                    .collect(),
                Err(e) => {
                    tracing::error!("Failed to fetch groups: {:?}", e);
                    vec![]
                }
            }
        } else {
            vec![]
        };

        ctrl.groups.set(groups);

        ctrl
    }

    pub fn get_groups(&self) -> Vec<GroupSummary> {
        (self.groups)()
    }

    pub async fn remove_group(&mut self, api: &GroupApi, group_id: String) {
        match api.remove_group(group_id).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to remove group: {e}");
            }
        };
    }

    pub async fn update_group_name(
        &mut self,
        api: &GroupApi,
        group_id: String,
        group_name: String,
    ) {
        match api.update_group_name(group_id, group_name).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to update group name: {e}");
            }
        };
    }
}
