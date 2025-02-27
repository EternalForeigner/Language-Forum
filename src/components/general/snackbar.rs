use dioxus::prelude::*;

#[component]
pub fn Snackbar(message: String) -> Element {
    rsx! {
        div { class: "fixed bottom-16 left-16 flex items-center justify-between bg-green-500/60 text-white px-8 py-6 rounded-lg shadow-lg space-x-4 z-50 animate-auto-fade",
            span { class: "text-sm font-medium whitespace-normal", {message} }
        }
    }
}
