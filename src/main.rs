use dioxus::prelude::*;

use components::Header;
use hooks::use_supabase_provider;
use require_auth::RequireAuth;
use views::{
    auth::{ForgotPassword, Login, Register},
    Home, Category, Profile,
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
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_supabase_provider(&env::APP_SUPABASE_URL, &env::APP_SUPABASE_KEY);

    rsx! {
        // Global app resources
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
