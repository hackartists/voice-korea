use dioxus::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[component]
pub fn UploadButton(
    class: String,
    text: String,
    onuploaded: EventHandler<FormEvent>,
    #[props(default = "image/*".to_string())] accept: String,
    #[props(default = false)] multiple: bool,
) -> Element {
    rsx! {
        input {
            id: "file-upload",
            class: "hidden",
            r#type: "file",
            accept,
            multiple,
            onchange: move |ev| {
                onuploaded.call(ev);
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
