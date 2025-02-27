use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaUser;
use dioxus_free_icons::Icon;
use supabase_rs::User;

use crate::components::general::Dropdown;
use crate::env::APP_SUPABASE_URL;
use crate::hooks::use_supabase;
use crate::Route;

const MENU_ITEM_CLASSES: &str = "cursor-pointer block transition hover:bg-zinc-950 py-4 px-8";

#[component]
pub fn AvatarMenu(user: User) -> Element {
    let nav = use_navigator();
    let mut has_avatar = use_signal(|| true);
    let avatar_url = format!(
        "{APP_SUPABASE_URL}/storage/v1/object/public/avatars/{}",
        user.id
    );

    let sign_out = move |_| async move {
        use_supabase().logout(None).await.unwrap();
        nav.push("/");
    };

    rsx! {
        Dropdown {
            button: rsx! {
                button { class: "cursor-pointer flex py-2 px-4 transition hover:bg-zinc-950",
                    if has_avatar() {
                        img {
                            class: "w-10 h-10 m-auto",
                            src: avatar_url,
                            onerror: move |_| has_avatar.set(false),
                        }
                    } else {
                        Icon { class: "w-10 h-10 m-auto", icon: FaUser }
                    }
                }
            },
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
