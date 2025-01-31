use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{
    GroupInfo, GroupResponse, InviteMemberRequest, ListMemberResponse, UpdateMemberRequest,
};

use crate::{
    api::common::CommonQueryResponse,
    models::role_field::RoleField,
    pages::members::page::AddMemberModal,
    service::{group_api::GroupApi, member_api::MemberApi, popup_service::PopupService},
};

use super::{i18n::MemberTranslate, page::RemoveMemberModal};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemberSummary {
    pub role_counts: Vec<u64>, // [전체 팀원, 관리자 수, 공론 관리자 수, 분석가 수, 중개자 수, 강연자 수],
    pub members: Vec<Member>,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Member {
    pub member_id: String,
    pub profile: Option<String>,
    pub profile_name: Option<String>,
    pub email: String,
    pub group: String,
    pub role: String,
    pub projects: Vec<String>, //유저가 속해있는 프로젝트
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub members: Signal<MemberSummary>,
    pub groups: Signal<Vec<GroupResponse>>,
    pub roles: Signal<Vec<RoleField>>,
    pub member_resource: Resource<Result<ListMemberResponse, ServerFnError>>,
    popup_service: Signal<PopupService>,
    member_api: MemberApi,
    pub group_resource: Resource<Result<CommonQueryResponse<GroupResponse>, ServerFnError>>,
}

impl Controller {
    pub fn init(lang: dioxus_translate::Language, popup_service: PopupService) -> Self {
        let translates: MemberTranslate = translate(&lang);
        let api: MemberApi = use_context();
        let member_resource: Resource<Result<ListMemberResponse, ServerFnError>> =
            use_resource(move || {
                let api = api.clone();
                async move { api.list_members(Some(100), None).await }
            });

        let group_api: GroupApi = use_context();
        let group_resource: Resource<Result<CommonQueryResponse<GroupResponse>, ServerFnError>> =
            use_resource(move || {
                let api = group_api.clone();
                async move { api.list_groups(Some(100), None).await }
            });

        let mut ctrl = Self {
            members: use_signal(|| MemberSummary::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    RoleField {
                        db_name: "super_admin".to_string(),
                        field: translates.manager.to_string(),
                    },
                    RoleField {
                        db_name: "public_admin".to_string(),
                        field: translates.public_opinion_manager.to_string(),
                    },
                    RoleField {
                        db_name: "analyst".to_string(),
                        field: translates.analyst.to_string(),
                    },
                    RoleField {
                        db_name: "mediator".to_string(),
                        field: translates.repeater.to_string(),
                    },
                    RoleField {
                        db_name: "speaker".to_string(),
                        field: translates.lecturer.to_string(),
                    },
                ]
            }),
            member_resource,
            member_api: api,
            popup_service: use_signal(|| popup_service),
            group_resource,
        };

        let groups = if let Some(v) = group_resource.value()() {
            match v {
                Ok(v) => v.items,
                Err(e) => {
                    tracing::error!("list groups failed: {:?}", e);
                    vec![]
                }
            }
        } else {
            vec![]
        };

        let (members, role_counts) = if let Some(v) = member_resource.value()() {
            match v {
                Ok(d) => {
                    (
                        d.members
                            .iter()
                            .map(|member| Member {
                                member_id: member.member.id.clone(),
                                profile: None,
                                profile_name: member.member.name.clone(),
                                email: member.member.email.clone(),
                                //FIXME: fix to group
                                group: if member.groups.len() == 0 {
                                    "".to_string()
                                } else {
                                    member.groups[0].name.clone()
                                },
                                role: if member.member.role.is_none() {
                                    "".to_string()
                                } else {
                                    member.member.role.clone().unwrap().to_string()
                                },
                                projects: vec![],
                            })
                            .collect(),
                        d.role_count,
                    )
                }
                Err(_) => (vec![], vec![]),
            }
        } else {
            (vec![], vec![])
        };

        ctrl.groups.set(groups);

        ctrl.members.set(MemberSummary {
            role_counts,
            members,
        });

        ctrl
    }

    pub fn get_members(&self) -> MemberSummary {
        (self.members)()
    }

    pub fn get_groups(&self) -> Vec<GroupResponse> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<RoleField> {
        (self.roles)()
    }

    pub async fn invite_member(&mut self, req: InviteMemberRequest) {
        let api: MemberApi = use_context();
        let _ = api.invite_member(req).await;
        self.member_resource.restart();
    }

    pub async fn update_group(&mut self, index: usize, group_index: usize) {
        let api: MemberApi = use_context();
        let members = self.get_members().members;
        let group = self.get_groups()[group_index].clone();
        let member = members[index].clone();
        let _ = api
            .update_member(
                member.member_id,
                UpdateMemberRequest {
                    name: member.profile_name,
                    group: Some(GroupInfo {
                        id: group.id,
                        name: group.name,
                    }), //FIXME: fix to real group
                    role: if member.role != "" {
                        Some(member.role)
                    } else {
                        None
                    },
                },
            )
            .await;
        self.member_resource.restart();
    }

    pub async fn update_role(&mut self, index: usize, role_name: String) {
        let api: MemberApi = use_context();
        let members = self.get_members().members;
        let member = members[index].clone();
        let _ = api
            .update_member(
                member.member_id,
                UpdateMemberRequest {
                    name: member.profile_name,
                    group: if member.group != "" {
                        Some(GroupInfo {
                            id: "group_id".to_string(),
                            name: member.group.clone(),
                        }) //FIXME: fix to real group
                    } else {
                        None
                    },
                    role: Some(role_name),
                },
            )
            .await;
        self.member_resource.restart();
    }

    pub async fn remove_member(&mut self, user_id: String) {
        let api: MemberApi = use_context();
        let _ = api.remove_member(user_id).await;
        self.member_resource.restart();
    }

    pub async fn open_remove_member_modal(
        &self,
        lang: Language,
        mut clicked_member_id: Signal<String>,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: MemberTranslate = translate(&lang);
        let api: MemberApi = self.member_api;

        let mut member_resource = self.member_resource;

        popup_service
            .open(rsx! {
                RemoveMemberModal {
                    lang,
                    remove_member: move |_e: MouseEvent| async move {
                        let _ = api.remove_member(clicked_member_id()).await;
                        member_resource.restart();
                        clicked_member_id.set("".to_string());
                        popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        clicked_member_id.set("".to_string());
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_team_member")
            .with_title(translates.remove_team_member);
    }

    pub async fn open_add_member_modal(&self, lang: Language) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: MemberTranslate = translate(&lang);
        let api: MemberApi = self.member_api;

        let mut member_resource = self.member_resource;
        let groups = (self.groups)();
        let roles = (self.roles)();

        popup_service
            .open(rsx! {
                AddMemberModal {
                    lang,
                    groups: groups.clone(),
                    roles: roles.clone(),
                    invite_member: move |req: InviteMemberRequest| async move {
                        let _ = api.invite_member(req).await;
                        member_resource.restart();
                        popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("add_team_member")
            .with_title(translates.add_team_member);
    }
}
