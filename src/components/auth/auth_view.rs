use dioxus::prelude::*;
use supabase_rs::EmailSignUpResult;

use crate::hooks::use_supabase;

#[derive(Clone, PartialEq)]
pub enum AuthView {
    Login,
    Register,
    ForgotPassword,
}

async fn login_submit(
    email: &str,
    password: &str,
    success_callback: Callback<String>,
    mut error_message: Signal<Option<String>>,
    nav: Navigator,
) {
    let supabase = use_supabase();
    let login_result = supabase.login_with_email(email, password).await;

    match login_result {
        Err(error) => error_message.set(Some(error.to_string())),
        _ => {
            success_callback.call(String::from("Login successful."));
            nav.push("/");
        }
    }
}

async fn register_submit(
    email: &str,
    password: &str,
    password_retype: &str,
    success_callback: Callback<String>,
    mut error_message: Signal<Option<String>>,
) {
    if password != password_retype {
        error_message.set(Some(String::from(
            "Please make sure your typed passwords match.",
        )));
        return;
    }

    let supabase = use_supabase();
    let register_result = supabase
        .sign_up_with_email_and_password(email, password)
        .await;

    match register_result {
        Err(error) => error_message.set(Some(error.to_string())),
        Ok(result) => match result {
            EmailSignUpResult::SessionResult(_) => success_callback(String::from(
                "Successfully registered. Please try logging in.",
            )),
            EmailSignUpResult::ConfirmationResult(_) => success_callback(String::from(
                "Please check your email for a confirmation link.",
            )),
        },
    }
}

async fn forgot_password_submit(
    email: &str,
    success_callback: Callback<String>,
    mut error_message: Signal<Option<String>>,
) {
    let supabase = use_supabase();
    let forgot_password_result = supabase.reset_password_for_email(email).await;

    match forgot_password_result {
        Err(error) => error_message.set(Some(error.to_string())),
        _ => success_callback(String::from(
            "Check your email for the password reset link.",
        )),
    }
}

impl AuthView {
    pub async fn submit(
        &self,
        email: &str,
        password: &str,
        password_retype: &str,
        success_callback: Callback<String>,
        error_message: Signal<Option<String>>,
        nav: Navigator,
    ) {
        match self {
            AuthView::Login => {
                login_submit(email, password, success_callback, error_message, nav).await
            }
            AuthView::Register => {
                register_submit(
                    email,
                    password,
                    password_retype,
                    success_callback,
                    error_message,
                )
                .await
            }
            AuthView::ForgotPassword => {
                forgot_password_submit(email, success_callback, error_message).await
            }
        }
    }
}
