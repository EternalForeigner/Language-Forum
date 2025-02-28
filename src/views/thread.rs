use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::{forum::ThreadPage, general::LoadingIndicator};

#[component]
pub fn Thread(thread_id: ReadOnlySignal<Uuid>) -> Element {
    rsx! {
        div { class: "container mx-auto py-4 px-8",
            SuspenseBoundary {
                fallback: |_| rsx! {
                    LoadingIndicator {}
                },
                ThreadPage { thread_id }
            }
        }
    }
}
