use dioxus::prelude::*;

use crate::models::Post as PostModel;

#[component]
pub fn Post(post: ReadOnlySignal<PostModel>) -> Element {
    rsx! {
        div { class: "px-2 py-2 min-w-full bg-black", {post().created_at.to_string()} }
        div { class: "px-4 py-4 min-w-full bg-gray-800", {post().text_body} }
    }
}
