use dioxus::prelude::*;

#[component]
pub fn ErrorNotice(message: String) -> Element {
    rsx! {
        div { class: "text-current text-sm mt-4 bg-red-400 rounded border border-red-700 p-4",
            {message}
        }
    }
}
