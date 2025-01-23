use chrono::{Local, TimeZone};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::UpdateMemberRequest;

use crate::service::member_api::MemberApi;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectType {
    #[default]
    Investigation, //조사
    PublicOpinion, //공론
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectStatus {
    #[default]
    Ready, //준비
    InProgress, //진행
    Finished,   //마감
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemberDetail {
    pub email: String,
    pub profile_image: Option<String>,
    pub profile_name: Option<String>,
    pub group: String,
    pub role: String,
    pub register_date: String,
    pub project_history: Vec<ProjectHistory>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectHistory {
    pub history_id: String,
    pub project_type: ProjectType,
    pub project_subject: String,
    pub role: String,
    pub panel: Vec<String>,
    pub period: String, //FIXME: fix start date, end date form
    pub project_status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub member: Signal<MemberDetail>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
    pub member_resource: Resource<Result<models::prelude::OrganizationMember, ServerFnError>>,
}

impl Controller {
    pub fn init(_lang: dioxus_translate::Language, member_id: String) -> Self {
        let api: MemberApi = use_context();
        let member_resource: Resource<Result<models::prelude::OrganizationMember, ServerFnError>> =
            use_resource(move || {
                let api = api.clone();
                let member_id = member_id.clone();
                async move { api.get_member(member_id.clone()).await }
            });
        let mut ctrl = Self {
            member: use_signal(|| MemberDetail::default()),
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

        let member = if let Some(v) = member_resource.value()() {
            match v {
                Ok(d) => {
                    let seconds = d.created_at / 1000;
                    let nanoseconds = (d.created_at % 1000) * 1_000_000;
                    let datetime = Local.timestamp_opt(seconds, nanoseconds as u32).unwrap();

                    let formatted_date = datetime.format("%Y년 %m월 %d일").to_string();

                    let data = MemberDetail {
                        email: d.email.clone(),
                        profile_image: None,
                        profile_name: d.name,
                        //FIXME: fix to group
                        group: "".to_string(),
                        role: if d.role.is_none() {
                            "".to_string()
                        } else {
                            d.role.unwrap().to_string()
                        },
                        register_date: formatted_date,
                        project_history: vec![],
                    };

                    tracing::debug!("member data: {:?}", data);
                    data
                }
                Err(_) => MemberDetail::default(),
            }
        } else {
            MemberDetail::default()
        };

        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        ctrl.member.set(member);

        ctrl
    }

    pub fn get_member(&self) -> MemberDetail {
        (self.member)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<String> {
        (self.roles)()
    }

    pub async fn update_member(&mut self, member_id: String, req: UpdateMemberRequest) {
        let api: MemberApi = use_context();
        let _ = match api.update_member(member_id, req).await {
            Ok(_) => self.member_resource.restart(),
            Err(v) => {
                tracing::error!("update failed: {v}");
            }
        };
    }

    pub async fn remove_member(&mut self, user_id: String) {
        let api: MemberApi = use_context();
        let _ = match api.remove_member(user_id).await {
            Ok(_) => self.member_resource.restart(),
            Err(v) => {
                tracing::error!("remove failed: {v}");
            }
        };
    }
}
