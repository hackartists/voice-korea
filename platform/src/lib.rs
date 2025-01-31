pub mod prelude {
    pub use crate::layouts::root_layout::*;
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
    pub use dioxus::document::eval;
}

pub mod pages {
    pub mod attributes;
    pub mod create;
    pub mod dashboard;
    pub mod find_email;
    pub mod id {
        pub mod response_report;
        pub mod select_response;
        pub mod survey_summary;
        pub mod write_question;
        pub mod write_title;
    }
    pub mod groups;
    pub mod login;
    pub mod members;
    pub mod not_found;
    pub mod opinions;
    pub mod panels;
    pub mod reset_password;
    pub mod resources;
    pub mod surveys;
}

pub mod service {
    pub mod attribute_api;
    pub mod auth_api;
    pub mod group_api;
    pub mod login_service;
    pub mod member_api;
    pub mod metadata_api;
    pub mod opinion_api;
    pub mod organization_api;
    pub mod panel_api;
    pub mod popup_service;
    pub mod prev_survey_api;
    pub mod survey_api;
    pub mod theme;
    pub mod user_api;
}

pub mod models {
    pub mod pi;
    pub mod question;
    pub mod role_field;
    pub mod survey;
    pub mod user;
}

pub mod utils {
    pub mod api;
    pub mod context;
    #[cfg(feature = "server")]
    pub mod db;
    pub mod hash;
    #[cfg(feature = "server")]
    pub mod logger;
    pub mod time;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod components {
    pub mod alert;
    pub mod bar_graph;
    pub mod bottom;
    pub mod button;
    pub mod calendar;
    pub mod checkbox;
    pub mod icons;
    pub mod input;
    pub mod label;
    pub mod pi_graph;
    pub mod popup;
    pub mod select;
    pub mod stepper;
    pub mod table_row;
    pub mod upload_button;
}

pub mod api;
pub mod routes;
