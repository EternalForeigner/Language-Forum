use dioxus::prelude::*;

use crate::components::{
    auth::{AuthUI, AuthView},
    PageContainer,
};

#[component]
pub fn Login() -> Element {
    rsx! {
        PageContainer {
            div { class: "max-w-lg mx-auto",
                AuthUI { view: AuthView::Login }
            }
        }
    }
}
