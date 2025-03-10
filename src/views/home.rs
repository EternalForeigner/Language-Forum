use dioxus::prelude::*;

use crate::{
    components::{
        forum::CategoriesTable,
        general::{ErrorNotice, LoadingIndicator},
        PageContainer,
    },
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
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator { extra_classes: "size-10 text-sky-500" }
                },
                CategoriesTable {}
            }
        }
    }
}
