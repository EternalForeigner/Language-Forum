use dioxus::prelude::*;

use crate::components::{forum::CategoryPage, general::LoadingIndicator, PageContainer};

#[component]
pub fn Category(category_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {}
                },
                CategoryPage { category_id }
            }
        }
    }
}
