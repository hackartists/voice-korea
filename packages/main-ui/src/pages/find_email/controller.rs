#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    name: Signal<String>,
    phone_number: Signal<String>,
    authentication_number: Signal<String>,

    email_address: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            name: use_signal(|| "".to_string()),
            phone_number: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            email_address: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_authentication_number(&self) -> String {
        (self.authentication_number)()
    }

    pub fn get_name(&self) -> String {
        (self.name)()
    }

    pub fn get_phone_number(&self) -> String {
        (self.phone_number)()
    }

    pub fn get_email_address(&self) -> String {
        (self.email_address)()
    }

    pub fn set_authentication_number(&mut self, authentication_number: String) {
        self.authentication_number.set(authentication_number);
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number.set(phone_number);
    }

    pub fn set_click_send_authentication(&mut self) {
        tracing::info!("send authentication button clicked");
    }

    //TODO: connect api
    pub fn find_email_address(&mut self) {
        tracing::info!("clicked: {}", self.get_email_address());
        self.email_address.set("miner@biyard.co".to_string());
        tracing::info!("email address: {}", self.get_email_address());
    }
}
