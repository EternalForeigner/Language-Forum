use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use supabase_rs::{Result, Supabase, SupabaseError};

use crate::{
    components::{
        general::{ErrorNotice, Snackbar, SubmitButton},
        INPUT_CLASSES,
    },
    hooks::use_supabase,
    models::{Category, Thread},
};

async fn post_thread(
    mut supabase: Supabase,
    category_id: i64,
    title: String,
    post_body: String,
) -> Result<Thread> {
    let json = serde_json::to_string(&NewThread { title, category_id })
        .map_err(|e| SupabaseError::Unknown(Box::new(e)))?;
    let response = supabase
        .from("threads")
        .await?
        .upsert(json)
        .execute()
        .await?
        .error_for_status()?;

    let thread = response
        .json::<Vec<Thread>>()
        .await?
        .first()
        .unwrap()
        .clone();

    let json = serde_json::to_string(&NewPost {
        text_body: post_body,
        thread_id: thread.id,
    })
    .map_err(|e| SupabaseError::Unknown(Box::new(e)))?;

    supabase
        .from("posts")
        .await?
        .upsert(json)
        .execute()
        .await?
        .error_for_status()?;

    Ok(thread)
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
struct NewThread {
    pub title: String,
    pub category_id: i64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
struct NewPost {
    pub thread_id: uuid::Uuid,
    pub text_body: String,
}

#[component]
pub fn NewThreadForm(category: Category) -> Element {
    let nav = use_navigator();
    let mut title = use_signal(|| String::new());
    let mut post_body = use_signal(|| String::new());

    let supabase = use_supabase();
    let mut is_submitting = use_signal(|| false);
    let mut snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);
    let mut error_message = use_signal(|| None);

    rsx! {
        form {
            onsubmit: move |_| {
                let supabase = supabase.clone();
                is_submitting.set(true);
                async move {
                    match post_thread(supabase, category.id, title(), post_body()).await {
                        Ok(thread) => {
                            snackbars.write().push(rsx! {
                                Snackbar { message: "Thread posted." }
                            });
                            nav.push(format!("/thread/{}", thread.id));
                        }
                        Err(error) => error_message.set(Some(error.to_string())),
                    }
                    is_submitting.set(false);
                }
            },
            p { class: "text-sm text-gray-300", {format!("Posting to: {}", category.name.clone())} }
            div { class: "my-8" }
            div { class: "flex flex-col space-y-4",
                input {
                    class: INPUT_CLASSES,
                    placeholder: "Thread Title...",
                    r#type: "text",
                    required: true,
                    value: title(),
                    oninput: move |e| title.set(e.value()),
                }
                textarea {
                    class: "{INPUT_CLASSES} my-6",
                    required: true,
                    value: post_body(),
                    oninput: move |e| post_body.set(e.value()),
                }
            }
            SubmitButton { is_loading: is_submitting(), "Save" }
            if let Some(error_message) = error_message() {
                ErrorNotice { message: error_message }
            }
        }
        for snackbar in snackbars() {
            {snackbar}
        }
    }
}
