use dioxus::prelude::*;

use crate::{hooks::use_supabase, Route};

#[component]
pub fn RequireAuth() -> Element {
    let nav = use_navigator();

    let supabase = use_supabase();

    let session = supabase.get_session_value();

    if let Some(session) = session {
        provide_context(session);
    } else {
        nav.replace("/login");
    }

    rsx! {
        Outlet::<Route> {}
    }
}
