use dioxus::prelude::*;

use crate::models::PostView;

#[component]
pub fn Post(post: ReadOnlySignal<PostView>) -> Element {
    rsx! {
        div { class: "px-2 py-2 min-w-full bg-black", {post().created_at.unwrap_or_default().to_string()} }
        div { class: "px-4 py-4 min-w-full bg-gray-800", {post().text_body} }
        div {{post().profile_name}}
    }
}
