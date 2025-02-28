use dioxus::prelude::*;

use crate::components::{forum::CategoryPage, general::LoadingIndicator};

#[component]
pub fn Category(category_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "container mx-auto py-4 px-8",
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {}
                },
                CategoryPage { category_id }
            }
        }
    }
}
