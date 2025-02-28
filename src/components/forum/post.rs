use dioxus::prelude::*;

use crate::{components::forum::PostProfileDisplay, models::PostView};

#[component]
pub fn Post(post_view: PostView) -> Element {
    rsx! {
        div { class: "px-2 py-2 min-w-full bg-black",
            {post_view.created_at.unwrap_or_default().to_string()}
        }
        div { class: "flex min-w-full bg-gray-800",
            PostProfileDisplay { post_view: post_view.clone() }
            div { class: "min-h-25 p-4",
                article {
                    blockquote { class: "", {post_view.text_body} }
                }
            }
        }
    }
}
