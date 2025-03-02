use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaUser;
use dioxus_free_icons::Icon;
use supabase_rs::User;

use crate::components::general::{Dropdown, ImageOrFallback};
use crate::helpers::get_avatar_url;
use crate::hooks::use_supabase;
use crate::Route;

const MENU_ITEM_CLASSES: &str = "cursor-pointer block transition hover:bg-zinc-950 py-4 px-8";

#[component]
pub fn AvatarMenu(user: ReadOnlySignal<User>) -> Element {
    let nav = use_navigator();
    let mut is_dropdown_open = use_signal(|| false);

    let sign_out = move |_| async move {
        use_supabase().logout(None).await.unwrap();
        nav.push("/");
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
                is_open: is_dropdown_open(),
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
    }
}
