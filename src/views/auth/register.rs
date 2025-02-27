use dioxus::prelude::*;

use crate::components::auth::{AuthUI, AuthView};

#[component]
pub fn Register() -> Element {
    rsx! {
        div { class: "container mx-auto py-4 px-8",
            div { class: "max-w-lg mx-auto",
                // AuthUI { view: AuthView::Register }
                "Registration coming soon"
            }
        }
    }
}
