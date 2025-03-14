use dioxus::prelude::*;

use crate::components::{forum::NewThreadPage, general::LoadingIndicator, PageContainer};

#[component]
pub fn CreateThread(category_id: i64) -> Element {
    rsx! {
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator { extra_classes: "size-10 text-sky-500" }
                },
                h1 { class: "my-2 text-3xl text-white", "Create Thread" }
                NewThreadPage { category_id }
            }
        }
    }
}
