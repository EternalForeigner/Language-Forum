use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use supabase_rs::{Result, Supabase, SupabaseError};
use uuid::Uuid;

use crate::{
    components::{
        forum::ThreadPosts,
        general::{ErrorNotice, Snackbar, SubmitButton},
        INPUT_CLASSES,
    },
    hooks::use_supabase,
    models::Thread as ThreadModel,
};

async fn get_thread(mut supabase: Supabase, id: Uuid) -> Result<Option<ThreadModel>> {
    let response = supabase
        .from("threads")
        .await?
        .eq("id", id.to_string())
        .execute()
        .await?
        .error_for_status()?;

    let list = response.json::<Vec<ThreadModel>>().await?;
    Ok(list.first().cloned())
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
struct NewPost {
    pub thread_id: uuid::Uuid,
    pub text_body: String,
}

async fn post_reply(mut supabase: Supabase, thread_id: Uuid, reply_text: String) -> Result<()> {
    let json = serde_json::to_string(&NewPost {
        text_body: reply_text,
        thread_id,
    })
    .map_err(|e| SupabaseError::Unknown(Box::new(e)))?;

    supabase
        .from("posts")
        .await?
        .upsert(json)
        .execute()
        .await?
        .error_for_status()?;

    Ok(())
}

#[component]
pub fn ThreadPage(thread_id: ReadOnlySignal<Uuid>) -> Element {
    let supabase = use_supabase();
    let is_logged_in = supabase.is_logged_in();

    let supabase_clone = supabase.clone();
    let thread = use_resource(move || get_thread(supabase_clone.clone(), thread_id())).suspend()?;

    let mut reply_text = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);
    let mut snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);
    let mut error_message = use_signal(|| None);

    rsx! {
        match &*thread.read() {
            Ok(thread) => {
                if let Some(thread) = thread {
                    rsx! {
                        h1 { class: "my-2 text-3xl text-gray-950 dark:text-white", {thread.title.clone()} }
                        // p { class: "text-sm text-gray-700 dark:text-gray-300", {thread..clone()} }
                        div { class: "my-8" }
                        ThreadPosts { thread: thread.clone() }
                        if is_logged_in {
                            form {
                                class: "flex flex-col my-6 space-y-2",
                                onsubmit: move |_| {
                                    let supabase = supabase.clone();
                                    is_submitting.set(true);
                                        async move {
                                            match post_reply(supabase, thread_id(), reply_text()).await {
                                                Ok(_) => {
                                                    snackbars.write().push(rsx! {
                                                        Snackbar { message: "Reply posted." }
                                                    });
                                                },
                                                Err(error) => error_message.set(Some(error.to_string())),
                                            }
                                            is_submitting.set(false);
                                        }
                                },
                                h2 { class: "text-lg text-gray-700 dark:text-gray-300", "Post Reply:" }
                                textarea {
                                    class: INPUT_CLASSES,
                                    required: true,
                                    value: reply_text(),
                                    oninput: move |e| reply_text.set(e.value()),
                                }
                                SubmitButton { is_loading: is_submitting(), extra_classes: "max-w-45", "Submit Reply" }
                                if let Some(error_message) = error_message() {
                                    ErrorNotice { message: error_message }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {
                        ErrorNotice { message: "Thread not found! Please check the URL." }
                    }
                }
            }
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        }
        for snackbar in snackbars() {
            {snackbar}
        }
    }
}
