use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaUpload, Icon};

use crate::components::BUTTON_CLASSES;

#[component]
pub fn FileUpload(
    accept: String,
    multiple: bool,
    onchange: Callback<Event<FormData>>,
    button_text: String,
) -> Element {
    rsx! {
        input {
            id: "select-file",
            class: "hidden",
            name: "select-file",
            r#type: "file",
            accept,
            multiple,
            onchange,
        }
        div { class: "flex flex-col items-center my-4",
            label {
                r#for: "select-file",
                class: "cursor-pointer flex flex-col items-center p-12 gap-y-4 text-white hover:text-blue-200",
                Icon { class: "w-12 h-12 ", icon: FaUpload }
                "Browse Files"
            }
        }
        div { class: "flex justify-center",
            button { class: BUTTON_CLASSES, r#type: "submit", {button_text} }
        }
    }
}
