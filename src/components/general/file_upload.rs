use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaUpload, Icon};

use crate::components::general::SubmitButton;

#[component]
pub fn FileUpload(
    accept: String,
    multiple: bool,
    onchange: Callback<Event<FormData>>,
    button_text: String,
    is_submitting: bool,
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
                class: "cursor-pointer flex flex-col items-center p-12 gap-y-4 text-gray-950 dark:text-white hover:text-sky-600 \
                    transition duration-300 ease-in-out",
                Icon { class: "w-12 h-12 ", icon: FaUpload }
                "Browse Files"
            }
        }
        div { class: "flex justify-center",
            SubmitButton { is_loading: is_submitting, {button_text} }
        }
    }
}
