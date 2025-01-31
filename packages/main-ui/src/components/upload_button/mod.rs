use dioxus::prelude::*;

#[cfg(feature = "web")]
use dioxus_logger::tracing;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[component]
pub fn UploadButton(
    class: String,
    text: String,
    onuploaded: Option<EventHandler<Vec<String>>>,
) -> Element {
    rsx! {
        input {
            id: "file-upload",
            class: "hidden",
            r#type: "file",
            accept: "image/*",
            multiple: false,
            onchange: move |_ev| async move {
                #[cfg(feature = "web")]
                if let Some(file_engine) = _ev.files() {
                    tracing::debug!("files: {:?}", file_engine.files());
                }
            },
        }
        button {
            class,
            onclick: move |_| {
                #[cfg(feature = "web")]
                {
                    let input = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("file-upload")
                        .unwrap();
                    input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().click();
                }
            },
            {text}
        }
    }
}
