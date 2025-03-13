use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::{forum::ThreadPage, general::LoadingIndicator, PageContainer};

#[component]
pub fn Thread(thread_id: ReadOnlySignal<Uuid>) -> Element {
    rsx! {
        PageContainer {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator { extra_classes: "size-10 text-sky-500" }
                },
                ThreadPage { thread_id }
            }
        }
    }
}
