use dioxus::prelude::*;

use components::Header;
use hooks::use_supabase_provider;
use require_auth::RequireAuth;
use uuid::Uuid;
use views::{
    auth::{ForgotPassword, Login, Register},
    Category, CreateThread, Home, Profile, Thread,
};

mod components;
mod env;
mod helpers;
mod hooks;
mod models;
mod require_auth;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},
    #[route("/category/:category_id")]
    Category {category_id: i64},
    #[route("/thread/:thread_id")]
    Thread {thread_id: Uuid},
    #[route("/create-thread/:category_id")]
    CreateThread {category_id: i64},

    #[layout(RequireAuth)]
    #[route("/profile")]
    Profile {},
    #[end_layout]
    
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/forgot-password")]
    ForgotPassword {}
}

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_supabase_provider(&env::APP_SUPABASE_URL, &env::APP_SUPABASE_KEY);

    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "min-h-screen flex flex-col", Router::<Route> {} }
    }
}
