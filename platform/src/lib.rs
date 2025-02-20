pub mod prelude {
    pub use crate::layouts::root_layout::*;
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
}

pub mod pages {
    pub mod not_found;
}

pub mod presentations {
    pub mod create;
    pub mod dashboard;
    pub mod find_email;
    pub mod login;
    pub mod reset_password;
    pub mod write_question;
    pub mod write_title;
}

pub mod models {
    pub mod question;
    pub mod survey;
}

pub mod utils {
    pub mod context;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod components {
    pub mod bottom;
    pub mod input;
    pub mod table_row;
}

pub mod api;
pub mod routes;
