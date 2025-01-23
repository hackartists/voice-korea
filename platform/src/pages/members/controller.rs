use dioxus::prelude::*;
use models::prelude::{GroupInfo, InviteMemberRequest, UpdateMemberRequest};

use crate::{api::common::CommonQueryResponse, service::member_api::MemberApi};

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

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub members: Signal<MemberSummary>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
    pub member_resource:
        Resource<Result<CommonQueryResponse<models::prelude::OrganizationMember>, ServerFnError>>,
}

impl Controller {
    pub fn init(_lang: dioxus_translate::Language) -> Self {
        let api: MemberApi = use_context();
        let member_resource: Resource<
            Result<CommonQueryResponse<models::prelude::OrganizationMember>, ServerFnError>,
        > = use_resource(move || {
            let api = api.clone();
            async move { api.list_members(Some(100), None).await }
        });
        let mut ctrl = Self {
            members: use_signal(|| MemberSummary::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    "관리자".to_string(),
                    "공론 관리자".to_string(),
                    "분석가".to_string(),
                    "중계자".to_string(),
                    "강연자".to_string(),
                ]
            }),
            member_resource,
        };

        let members = if let Some(v) = member_resource.value()() {
            match v {
                Ok(d) => d
                    .items
                    .iter()
                    .map(|member| Member {
                        member_id: member.id.clone(),
                        profile: None,
                        profile_name: member.name.clone(),
                        email: member.email.clone(),
                        group: "".to_string(),
                        role: if member.role.is_none() {
                            "".to_string()
                        } else {
                            member.role.clone().unwrap().to_string()
                        },
                        projects: vec![],
                    })
                    .collect(),
                Err(_) => vec![],
            }
        } else {
            vec![]
        };

        let mut role_counts = vec![0, 0, 0, 0, 0, 0];

        for member in members.clone() {
            role_counts[0] += 1;
            if member.role == "관리자" {
                role_counts[1] += 1;
            } else if member.role == "공론 관리자" {
                role_counts[2] += 1;
            } else if member.role == "분석가" {
                role_counts[3] += 1;
            } else if member.role == "중계자" {
                role_counts[4] += 1;
            } else if member.role == "강연자" {
                role_counts[5] += 1;
            }
        }

        ctrl.members.set(MemberSummary {
            role_counts,
            members,
        });

        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        ctrl
    }

    pub fn get_members(&self) -> MemberSummary {
        (self.members)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<String> {
        (self.roles)()
    }

    pub async fn invite_member(&mut self, req: InviteMemberRequest) {
        let api: MemberApi = use_context();
        let _ = api.invite_member(req).await;
        self.member_resource.restart();
    }

    pub async fn update_group(&mut self, index: usize, group_name: String) {
        let api: MemberApi = use_context();
        let members = self.get_members().members;
        let member = members[index].clone();
        let _ = api
            .update_member(
                member.member_id,
                UpdateMemberRequest {
                    name: member.profile_name,
                    group: Some(GroupInfo {
                        id: "group_id".to_string(),
                        name: group_name.clone(),
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
}
