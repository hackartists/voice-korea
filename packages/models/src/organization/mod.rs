use crate::member::CreateMemberRequest;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub gsi1: String,  //user_id
    pub gsi2: String,  //user_id#organization_id
    pub email: String, // FIXME: remove this field if postgre is implemented
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub user_id: String, // FIXME: change this field to user_email if postgre is implemented (Foreign Key)
    pub organization_id: String,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub contact: Option<String>,
}

impl OrganizationMember {
    pub fn new(
        user_id: String,
        organization_id: String,
        email: String,
        name: Option<String>,
        role: Option<Role>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();

        OrganizationMember {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: OrganizationMember::get_type(),
            gsi1: OrganizationMember::get_gsi1(&email),
            gsi2: OrganizationMember::get_gsi2(&email, &organization_id),
            user_id,
            organization_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name,
            role,
            email,
            contact: None,
        }
    }

    pub fn get_gsi1(email: &str) -> String {
        format!("{}#{}", Self::get_type(), email)
    }

    pub fn get_gsi2(email: &str, organization_id: &str) -> String {
        format!("{}#{}#{}", Self::get_type(), email, organization_id)
    }

    pub fn get_gsi1_deleted(email: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), email)
    }

    pub fn get_gsi2_deleted(email: &str, organization_id: &str) -> String {
        format!("{}#{}#{}", Self::get_deleted_type(), email, organization_id)
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
            gsi1: OrganizationMember::get_gsi1(&req.email),
            gsi2: OrganizationMember::get_gsi2(&req.email, &organization_id),
            user_id,
            organization_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: req.name,
            role: req.role,
            email: req.email,
            contact: None,
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub struct Organization {
//     pub id: String,
//     pub r#type: String,
//     pub gsi1: String,
//     pub created_at: i64,
//     pub updated_at: i64,
//     pub deleted_at: Option<i64>,

//     pub name: String,
//     pub user_id: String,
// }

// impl Organization {
//     pub fn new(user_id: String, email_address: String) -> Self {
//         let now = chrono::Utc::now().timestamp_millis();
//         let id = uuid::Uuid::new_v4().to_string();
//         Self {
//             id: id.clone(),
//             r#type: Self::get_type(),
//             gsi1: Self::get_gsi1(&id),
//             created_at: now,
//             updated_at: now,
//             deleted_at: None,
//             name: email_address,
//             user_id,
//         }
//     }

//     pub fn get_gsi1(id: &str) -> String {
//         format!("{}#{}", Self::get_type(), id)
//     }

//     pub fn get_gsi1_deleted(id: &str) -> String {
//         format!("{}#{}", Self::get_deleted_type(), id)
//     }

//     pub fn get_deleted_type() -> String {
//         "deleted#organization".to_string()
//     }

//     pub fn get_type() -> String {
//         "organization".to_string()
//     }
// }

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
