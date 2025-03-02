use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaUser;
use dioxus_free_icons::Icon;
use supabase_rs::User;

use crate::components::general::{Dropdown, ErrorNotice, ImageOrFallback};
use crate::helpers::get_avatar_url;
use crate::hooks::use_supabase;
use crate::Route;

const MENU_ITEM_CLASSES: &str = "cursor-pointer block transition hover:bg-zinc-950 py-4 px-8";

#[component]
pub fn AvatarMenu(user: ReadOnlySignal<User>) -> Element {
    let nav = use_navigator();
    let supabase = use_supabase();
    let mut error_message = use_signal(|| None);

    let mut is_dropdown_open = use_signal(|| false);

    let sign_out = move |_| {
        let supabase_clone = supabase.clone();
        async move {
            let logout_result = supabase_clone.logout(None).await;
            match logout_result {
                Ok(_) => {
                    nav.push("/");
                }
                Err(e) => error_message.set(Some(e.to_string())),
            }
        }
    };

    rsx! {
        div { class: "group",
            button {
                class: "cursor-pointer flex py-2 px-4 transition hover:bg-zinc-950",
                onclick: move |_| is_dropdown_open.set(!is_dropdown_open()),
                ImageOrFallback {
                    image_class: "w-10 h-10 m-auto",
                    image_url: get_avatar_url(user().id),
                    fallback: rsx! {
                        Icon { class: "w-10 h-10 m-auto", icon: FaUser }
                    },
                }
            }
            Dropdown {
                is_active: is_dropdown_open,
                menu: rsx! {
                    ul { class: "bg-zinc-700",
                        li {
                            Link { class: "{MENU_ITEM_CLASSES} ", to: Route::Profile {}, "Profile" }
                        }
                        li {
                            button { class: "{MENU_ITEM_CLASSES}", onclick: sign_out, "Sign out" }
                        }
                    }
                },
            }
        }
        // TODO: error snackbar would be better
        if let Some(error_message) = error_message() {
            ErrorNotice { message: error_message }
        }
    }
}
