use dioxus::prelude::*;

use crate::models::PostView;

#[component]
pub fn PostProfileDisplay(post: ReadOnlySignal<PostView>) -> Element {
    rsx! {
        div { class: "p-10", 
        
        {post().profile_name.unwrap_or("[Deleted user]".into())} }
    }
}
