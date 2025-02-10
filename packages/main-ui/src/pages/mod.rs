mod controller;
mod i18n;
mod page;

pub mod attributes;
pub mod create;
pub mod find_email;
pub mod id {
    pub mod response_report;
    pub mod select_response;
    pub mod survey_summary;
    pub mod write_question;
    pub mod write_title;
}
pub mod groups;
pub mod members;
pub mod not_found;
pub mod opinions;
pub mod panels;
pub mod reset_password;
pub mod resources;
pub mod surveys;

pub use page::*;
