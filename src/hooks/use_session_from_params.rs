use dioxus::prelude::*;

use super::{try_get_session_from_params, use_supabase};

pub fn use_session_from_params(redirect_url: Option<String>) -> Signal<Option<String>> {
    let nav = use_navigator();
    let supabase = use_supabase();
    let mut recovery_session = use_signal(|| None);
    let mut error_message = use_signal(|| None);
    use_future(move || async move {
        try_get_session_from_params(&mut recovery_session, &mut error_message).await;
    });

    use_effect(move || {
        if let Some(recovery_session) = recovery_session() {
            supabase.set_session_value(Some(recovery_session));
            if let Some(redirect_url) = redirect_url.clone() {
                nav.push(redirect_url);
            }
        }
    });

    error_message
}
