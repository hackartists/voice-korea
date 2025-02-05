use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Props, Clone, PartialEq)]
pub struct SettingPanelProps {
    lang: Language,
}

#[component]
pub fn SettingPanel(props: SettingPanelProps) -> Element {
    let _props = props;
    rsx! {
        div { "Hello" }
    }
}
