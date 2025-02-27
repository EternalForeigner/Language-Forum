use dioxus::prelude::*;

use crate::{
    components::{auth::{AuthUI, AuthView}, general::ErrorNotice},
    hooks::use_session_from_params,
};

#[component]
pub fn ForgotPassword() -> Element {
    let error_message = use_session_from_params(Some("/account-settings/password".to_string()));

    rsx! {
        div { class: "container mx-auto py-4 px-8",
            div { class: "flex flex-col space-y-4 max-w-lg mx-auto",
                AuthUI { view: AuthView::ForgotPassword }
                if let Some(error_message) = error_message() {
                    ErrorNotice { message: error_message }
                }
            }
        }
    }
}
