use dioxus::prelude::*;

use crate::{
    components::{forum::CategoriesTable, general::{ErrorNotice, LoadingIndicator}},
    hooks::use_session_from_params,
};

#[component]
pub fn Home() -> Element {
    let error_message = use_session_from_params(None);

    if let Some(error_message) = error_message() {
        return rsx! {
            ErrorNotice { message: error_message }
        };
    }

    rsx! {
        div { class: "container mx-auto py-4 px-8",
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {}
                },
                CategoriesTable {}
            }
        }
    }
}
