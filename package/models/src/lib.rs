mod attribute;
mod field;
mod group;
mod member;
mod metadata;
mod organization;
mod pagination;
mod panel;
mod public_opinion;
mod public_survey;
mod search;
mod survey;
mod update_field;
mod user;
pub mod error;
mod auth;

pub use crate::user::{AuthDocument, User};
pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::field::*;
    pub use crate::group::*;
    pub use crate::member::*;
    pub use crate::metadata::*;
    pub use crate::organization::*;
    pub use crate::pagination::*;
    pub use crate::panel::*;
    pub use crate::public_opinion::*;
    pub use crate::public_survey::*;
    pub use crate::search::*;
    pub use crate::survey::*;
    pub use crate::update_field::*;
    pub use crate::user::*;
    pub use crate::error::*;
    pub use crate::auth::*;
}

pub type Result<T> = std::result::Result<T, crate::error::ApiError>;