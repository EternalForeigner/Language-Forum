use std::rc::Rc;
use supabase_rs::{Session, Supabase};

use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

pub fn use_supabase_provider(url: &str, api_key: &str) {
    let session: Signal<Option<Session>> = use_persistent("session", || None);

    let supabase = Supabase::new(
        url,
        api_key,
        "",
        Rc::new(move || session()),
        // This is the cause of the really long warning. But it doesn't actually
        // seem to create any issues as long as the write doesn't cause an infinite loop
        Rc::new(move |s| *session.write_unchecked() = s),
    );
    use_context_provider(|| supabase);
}

pub fn use_supabase() -> Supabase {
    use_context::<Supabase>()
}
