use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use crate::routes::Route;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IitpContext {
    pub lang: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum Language {
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "en")]
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Ko => write!(f, "ko"),
            Language::En => write!(f, "en"),
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ko" => Ok(Language::Ko),
            "en" => Ok(Language::En),
            _ => Err(format!("Invalid language: {}", s)),
        }
    }
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::Ko => "ko".to_string(),
            Language::En => "en".to_string(),
        }
    }
}

pub fn use_iitp_context_provider() {
    // FIXME: initialized with default lang or url.
    // This will be initialized language to ko even if a user access the site with /en.
    use_context_provider(|| Signal::new(IitpContext { lang: Language::Ko }));
}

pub fn use_iitp_context() -> Signal<IitpContext> {
    use_context()
}

pub fn use_iitp_context_lang() -> Language {
    use_iitp_context().cloned().lang
}

pub fn default_lang() -> Language {
    Language::Ko
}

#[derive(Clone)]
pub struct LoginPopupState(pub bool, pub Option<Route>);

pub fn use_login_context_provider() {
    use_context_provider(|| Signal::new(LoginPopupState(false, None)));
}
