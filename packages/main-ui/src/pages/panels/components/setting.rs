use dioxus::prelude::*;

#[component]
pub fn AttributeSetting(
    onsave: EventHandler<String>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { "Attribute Setting" }
    }
}
