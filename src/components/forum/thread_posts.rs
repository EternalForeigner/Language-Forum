use dioxus::prelude::*;
use supabase_rs::Result;
use uuid::Uuid;

use crate::{
    components::{forum::Post, general::ErrorNotice},
    hooks::use_supabase,
    models::{Post as PostModel, Thread as ThreadModel},
};

async fn get_posts(thread_id: Uuid) -> Result<Vec<PostModel>> {
    let response = use_supabase()
        .from("posts")
        .await?
        .eq("thread_id", thread_id.to_string())
        .order("created_at.asc")
        .execute()
        .await?
        .error_for_status()?;

    Ok(response.json::<Vec<PostModel>>().await?)
}

#[component]
pub fn ThreadPosts(thread: ReadOnlySignal<ThreadModel>) -> Element {
    let posts_result = use_resource(move || get_posts(thread().id)).suspend()?;

    rsx! {
        div { class: "min-w-full border border-white",
            match &*posts_result.read() {
                Ok(posts) => rsx! {
                    for post in posts {
                        Post { post: post.clone()}
                    }
                },
                Err(error) => rsx! {
                    ErrorNotice { message: error.to_string() }
                },
            }
        }
    }
}
