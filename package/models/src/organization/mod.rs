use serde::{Deserialize, Serialize};
use crate::member::CreateMemberRequest;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OrganizationMiddlewareParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OrganizationMemberResponse {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub user_id: String,
    pub organization_id: String,
    pub organization_name: String,
    pub creator: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OrganizationMember {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, //user_id
    pub gsi2: String, //user_id#organization_id
    pub email: String, // FIXME: remove this field if postgre is implemented
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub user_id: String,
    pub organization_id: String,
    pub name: Option<String>,
    pub role: Option<Role>,
}

impl OrganizationMember {
    pub fn new(id: String, user_id: String, organization_id: String, role: Option<Role>) -> Self {
        //uuid, user_id, organization_id
        let mut organization_member = OrganizationMember::default();
        let now = chrono::Utc::now().timestamp_millis();
        organization_member.id = id;
        organization_member.r#type = OrganizationMember::get_type();
        organization_member.gsi1 = OrganizationMember::get_gsi1(&user_id);
        organization_member.gsi2 = OrganizationMember::get_gsi2(&user_id, &organization_id);
        organization_member.created_at = now;
        organization_member.updated_at = now;
        organization_member.deleted_at = None;

        organization_member.user_id = user_id;
        organization_member.organization_id = organization_id;
    
        if let Some(r) = role {
            organization_member.role = Some(r);
        };

        organization_member
    }

    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi2(user_id: &str, organization_id: &str) -> String {
        format!("{}#{}#{}", Self::get_type(), user_id, organization_id)
    }

    pub fn get_gsi1_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_gsi2_deleted(user_id: &str, organization_id: &str) -> String {
        format!(
            "{}#{}#{}",
            Self::get_deleted_type(),
            user_id,
            organization_id
        )
    }

    pub fn get_deleted_type() -> String {
        "deleted#organization#member".to_string()
    }

    pub fn get_type() -> String {
        "organization#member".to_string()
    }
}

impl Into<OrganizationMember> for (CreateMemberRequest, String, String, String) {
    fn into(self) -> OrganizationMember {
        let (req, id, user_id, organization_id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        OrganizationMember {
            id,
            r#type: OrganizationMember::get_type(),
            gsi1: OrganizationMember::get_gsi1(&user_id),
            gsi2: OrganizationMember::get_gsi2(&user_id, &organization_id),
            user_id,
            organization_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: req.name,
            role: req.role,
            email: req.email,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Organization {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub user_id: String,
}

impl Organization {
    pub fn new(id: String, user_id: String, email_address: String) -> Self {
        let mut organization = Organization::default();
        let now = chrono::Utc::now().timestamp_millis();
        organization.id = id;
        organization.r#type = Organization::get_type();
        organization.gsi1 = Organization::get_gsi1(&email_address);
        organization.created_at = now;
        organization.updated_at = now;
        organization.deleted_at = None;
        organization.user_id = user_id;
        organization.name = email_address;

        organization
    }

    pub fn get_gsi1(id: &str) -> String {
        format!("{}#{}", Self::get_type(), id)
    }

    pub fn get_gsi1_deleted(id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#organization".to_string()
    }

    pub fn get_type() -> String {
        "organization".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Role {
    #[serde(rename = "super_admin")]
    Admin,
    #[serde(rename = "public_admin")]
    PublicAdmin,
    #[serde(rename = "analyst")]
    Analyst,
    #[serde(rename = "mediator")]
    Mediator,
    #[serde(rename = "speaker")]
    Speaker,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "super_admin"),
            Role::PublicAdmin => write!(f, "public_admin"),
            Role::Analyst => write!(f, "analyst"),
            Role::Mediator => write!(f, "mediator"),
            Role::Speaker => write!(f, "speaker"),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(r: &str) -> Result<Self, Self::Err> {
        match r {
            "super_admin" => Ok(Role::Admin),
            "public_admin" => Ok(Role::PublicAdmin),
            "analyst" => Ok(Role::Analyst),
            "mediator" => Ok(Role::Mediator),
            "speaker" => Ok(Role::Speaker),
            _ => Err("Invalid role".to_string()),
        }
    }
}
