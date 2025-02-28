use dioxus::prelude::*;
use supabase_rs::Result;
use uuid::Uuid;

use crate::{
    components::{forum::ThreadPosts, general::ErrorNotice},
    hooks::use_supabase,
    models::Thread as ThreadModel,
};

async fn get_thread(id: Uuid) -> Result<Option<ThreadModel>> {
    let response = use_supabase()
        .from("threads")
        .await?
        .eq("id", id.to_string())
        .execute()
        .await?
        .error_for_status()?;

    let list = response.json::<Vec<ThreadModel>>().await?;
    Ok(list.first().cloned())
}

#[component]
pub fn ThreadPage(thread_id: ReadOnlySignal<Uuid>) -> Element {
    let thread = use_resource(move || get_thread(thread_id())).suspend()?;

    rsx! {
        match &*thread.read() {
            Ok(thread) => {
                if let Some(thread) = thread {
                    rsx! {
                        h1 { class: "my-2 text-3xl text-white", {thread.title.clone()} }
                        // p { class: "text-sm text-gray-300", {thread..clone()} }
                        div { class: "my-8" }
                        ThreadPosts { thread: thread.clone() }
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
    }
}
