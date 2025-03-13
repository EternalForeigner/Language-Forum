use dioxus::prelude::*;

use crate::components::{
    auth::{AuthUI, AuthView},
    PageContainer,
};

#[component]
pub fn Register() -> Element {
    rsx! {
        PageContainer {
            div { class: "max-w-lg mx-auto",
                AuthUI { view: AuthView::Register }
            }
        }
    }
}
