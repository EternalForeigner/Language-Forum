use dioxus::prelude::*;
use supabase_rs::Provider;

use crate::{
    components::{
        auth::OauthButton,
        general::{ErrorNotice, Snackbar, SubmitButton},
        INPUT_CLASSES, LABEL_CLASSES, LINK_CLASSES,
    },
    Route,
};

use super::AuthView;

#[component]
pub fn AuthUI(view: AuthView) -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut password_retype = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);
    let error_message: Signal<Option<String>> = use_signal(|| None);
    let mut snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col space-y-4",
            if &view == &AuthView::Login || &view == &AuthView::Register {
                div { class: "space-y-4",
                    OauthButton { provider: Provider::Google }
                    OauthButton { provider: Provider::Discord }
                }
            }
            form {
                class: "space-y-8",
                onsubmit: move |_| {
                    is_submitting.set(true);
                    let view_clone = view.clone();
                    async move {
                        let _ = &view_clone
                            .submit(
                                &email(),
                                &password(),
                                &password_retype(),
                                Callback::new(move |s| {
                                    snackbars.write().push(rsx! {
                                        Snackbar { message: s }
                                    })
                                }),
                                error_message,
                                nav,
                            )
                            .await;
                        is_submitting.set(false);
                    }
                },
                div { class: "flex flex-col space-y-4",
                    label { class: LABEL_CLASSES, "Email" }
                    input {
                        class: INPUT_CLASSES,
                        r#type: "email",
                        value: email,
                        required: true,
                        placeholder: "Your email address",
                        oninput: move |e| email.set(e.value()),
                    }
                    if &view != &AuthView::ForgotPassword {
                        label { class: LABEL_CLASSES, "Password" }
                        input {
                            class: INPUT_CLASSES,
                            r#type: "password",
                            value: password,
                            required: true,
                            placeholder: "Your password",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    if &view == &AuthView::Register {
                        label { class: LABEL_CLASSES, "Retype Password" }
                        input {
                            class: INPUT_CLASSES,
                            r#type: "password",
                            value: password_retype,
                            required: true,
                            placeholder: "Your password again",
                            oninput: move |e| password_retype.set(e.value()),
                        }
                    }
                    if let Some(error_message) = error_message() {
                        ErrorNotice { message: error_message }
                    }
                }
                SubmitButton { is_loading: is_submitting(),
                    match &view {
                        &AuthView::Login => "Log in",
                        &AuthView::Register => "Sign up",
                        &AuthView::ForgotPassword => "Send reset password instructions",
                    }
                }
            }
            if &view == &AuthView::Login {
                Link { class: "{LINK_CLASSES}", to: Route::ForgotPassword {}, "Forgot your password?" }
            }
            match &view {
                &AuthView::Login => rsx! {
                    Link { class: "{LINK_CLASSES}", to: Route::Register {},
                        "Don't have an account yet? Click here to sign up."
                    }
                },
                &AuthView::Register | &AuthView::ForgotPassword => {
                    rsx! {
                        Link { class: "{LINK_CLASSES} ", to: Route::Login {}, "Already have an account? Click here to log in." }
                    }
                }
            }
        }
        for snackbar in snackbars() {
            {snackbar}
        }
    }
}
