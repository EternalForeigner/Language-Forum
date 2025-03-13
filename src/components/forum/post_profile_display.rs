use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaUser, Icon};

use crate::{components::general::ImageOrFallback, helpers::get_avatar_url, models::PostView};

#[component]
pub fn PostProfileDisplay(post_view: PostView) -> Element {
    let avatar_classes = "w-30 h-30 m-auto";

    rsx! {
        div { class: "px-10 py-6",
            div { class: "p-2",
                ImageOrFallback {
                    image_class: avatar_classes,
                    image_url: post_view.profile_id.map(|id| get_avatar_url(id)),
                    fallback: rsx! {
                        Icon { class: avatar_classes, icon: FaUser }
                    },
                }
            }
            div { class: "m-auto text-center", {post_view.profile_name.unwrap_or("[Deleted user]".into())} }
        }
    }
}
