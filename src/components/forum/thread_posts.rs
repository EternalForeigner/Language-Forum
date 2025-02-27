use dioxus::prelude::*;

use crate::models::Thread as ThreadModel;

#[component]
pub fn ThreadPosts(thread: ReadOnlySignal<ThreadModel>) -> Element {
    rsx! {
        div {
            class: "min-w-full border border-white",
            div {
                class: "px-2 py-2 min-w-full bg-black",
                {thread().created_at.to_string()}
            }
        }
    }
}
