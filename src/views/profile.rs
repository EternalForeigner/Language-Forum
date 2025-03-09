use dioxus::prelude::*;

use crate::components::{general::LoadingIndicator, profile::ProfilePage, PageContainer};

#[component]
pub fn Profile() -> Element {
    rsx! {
        PageContainer {
            div { class: "max-w-lg mx-auto",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        LoadingIndicator {}
                    },
                    ProfilePage {}
                }
            }
        }
    }
}
