use dioxus::prelude::*;

pub use dioxus_translate::Language;
use serde::{Deserialize, Serialize};

use crate::routes::Route;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IitpContext {
    pub lang: Language,
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

#[derive(Clone)]
pub struct LoginPopupState(pub bool, pub Option<Route>);

pub fn use_login_context_provider() {
    use_context_provider(|| Signal::new(LoginPopupState(false, None)));
}
