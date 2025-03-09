use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::{forum::ThreadPage, general::LoadingIndicator, PageContainer};

#[component]
pub fn Thread(thread_id: ReadOnlySignal<Uuid>) -> Element {
    rsx! {
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {}
                },
                ThreadPage { thread_id }
            }
        }
    }
}
