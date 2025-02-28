use dioxus::prelude::*;

use crate::{components::forum::PostProfileDisplay, models::PostView};

#[component]
pub fn Post(post: ReadOnlySignal<PostView>) -> Element {
    rsx! {
        div { class: "px-2 py-2 min-w-full bg-black",
            {post().created_at.unwrap_or_default().to_string()}
        }
        div { class: "flex min-w-full bg-gray-800",
            PostProfileDisplay { post: post() }
            div { {post().text_body} }
        }
    }
}
