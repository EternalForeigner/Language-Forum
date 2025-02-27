use crate::{components::auth::AvatarMenu, hooks::use_supabase, Route};
use dioxus::prelude::*;

// const LOGO_CSS: Asset = asset!("/assets/styling/logo.css");
const LINK_CLASSES: &str = "transition hover:bg-zinc-950";

fn get_auth_links() -> Result<VNode, RenderError> {
    rsx! {
        div { class: "flex",
            Link {
                class: "{LINK_CLASSES} content-center py-2 px-4",
                to: Route::Login {},
                "Log in"
            }
            Link {
                class: "{LINK_CLASSES} content-center py-2 px-4",
                to: Route::Register {},
                "Sign up"
            }
        }
    }
}

#[component]
pub fn Header() -> Element {
    let user = use_supabase().get_session_value().map(|s| s.user);
    let right_side_content = if let Some(user) = user {
        rsx! {
            AvatarMenu { user }
        }
    } else {
        get_auth_links()
    };

    rsx! {
        // document::Link { rel: "stylesheet", href: LOGO_CSS }

        div { class: "flex p-0 bg-zinc-900",
            Link {
                class: "{LINK_CLASSES} logo-background-transition flex py-2 px-4",
                to: Route::Home {},
                div { class: "flex px-4",
                    span { class: "span-svg-icon gradient gradient-svg m-auto" }
                }
                div { class: "gradient gradient-text white-outline-text text-2xl content-center",
                    "Language Forum"
                }
            }
            div { class: "flex-1" }
            {right_side_content}
        }

        Outlet::<Route> {}
    }
}
