use serde::{Deserialize, Serialize};
use crate::member::GroupInfo;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupProject {
    pub project_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupMember {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub gsi2: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub group_id: String,
    pub org_member_id: String,
}

impl GroupMember {
    pub fn new(id: String, group_id: String, org_member_id: String) -> Self {
        let mut group = GroupMember::default();
        let now = chrono::Utc::now().timestamp_millis();
        group.id = id;
        group.r#type = GroupMember::get_type();
        group.gsi1 = GroupMember::get_gsi1(&group_id);
        group.gsi2 = GroupMember::get_gsi2(&org_member_id);
        group.created_at = now;
        group.updated_at = now;
        group.deleted_at = None;

        group.group_id = group_id;
        group.org_member_id = org_member_id;
        group
    }

    pub fn get_gsi1(group_id: &str) -> String {
        format!("{}#{}", Self::get_type(), group_id)
    }

    pub fn get_gsi2(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi1_deleted(group_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), group_id)
    }

    pub fn get_gsi2_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#group#member".to_string()
    }

    pub fn get_type() -> String {
        "group#member".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MemberInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupMemberResponse {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub group_id: String,
    pub org_member_id: String,
    pub user_name: String,
    pub user_email: String,
    pub role_name: Option<String>,
    pub group_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupResponse {
    pub id: String,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub members: Vec<GroupMemberResponse>,
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Group {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
    pub organization_id: String,
}

impl Group {
    pub fn new(id: String, user_id: String) -> Self {
        let mut group = Group::default();
        let now = chrono::Utc::now().timestamp_millis();
        group.id = id;
        group.r#type = Group::get_type();
        group.gsi1 = Group::get_gsi1(&user_id);
        group.creator = user_id;
        group.created_at = now;
        group.updated_at = now;
        group.deleted_at = None;
        group
    }

    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#group".to_string()
    }

    pub fn get_type() -> String {
        "group".to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateGroupMember {
    pub member_id: String,
    pub member_name: String,
    pub member_email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateGroupRequest {
    pub name: String,
    pub members: Vec<CreateGroupMember>,            //그룹 내 팀원
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

impl Into<Group> for (CreateGroupRequest, String, String, String) {
    fn into(self) -> Group {
        let (req, id, user_id, org_id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        Group {
            id,
            r#type: Group::get_type(),
            gsi1: Group::get_gsi1(&user_id),
            creator: user_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: req.name,
            public_opinion_projects: req.public_opinion_projects,
            investigation_projects: req.investigation_projects,
            organization_id: org_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TeamMemberRequest {
    pub member_id: String,
    pub email: String,
    pub name: Option<String>,
    pub group: Option<GroupInfo>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum GroupActionRequest {
    Create(CreateGroupRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum GroupByIdActionRequest {
    UpdateName(String),
    Delete,
    AddTeamMember(TeamMemberRequest),
    RemoveTeamMember(String),
}
