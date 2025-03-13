use dioxus::prelude::*;

use crate::components::{forum::CategoryPage, general::LoadingIndicator, PageContainer};

#[component]
pub fn Category(category_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {extra_classes: "size-10 text-sky-500"}
                },
                CategoryPage { category_id }
            }
        }
    }
}
