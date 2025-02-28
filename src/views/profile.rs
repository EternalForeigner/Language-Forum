use dioxus::prelude::*;

use crate::components::{forum::ProfilePage, general::LoadingIndicator};

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "container mx-auto py-4 px-8",
            div { class: "max-w-lg mx-auto",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        LoadingIndicator {}
                    },
                    ProfilePage {  }
                }
            }
        }
    }
}
