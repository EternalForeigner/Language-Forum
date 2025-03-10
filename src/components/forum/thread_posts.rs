use dioxus::prelude::*;
use supabase_rs::Result;
use uuid::Uuid;

use crate::{
    components::{forum::Post, general::ErrorNotice},
    hooks::use_supabase,
    models::{PostView, Thread as ThreadModel},
};

async fn get_posts(thread_id: Uuid) -> Result<Vec<PostView>> {
    let response = use_supabase()
        .from("post_view")
        .await?
        .eq("thread_id", thread_id.to_string())
        .order("created_at.asc")
        .execute()
        .await?
        .error_for_status()?;

    Ok(response.json::<Vec<PostView>>().await?)
}

#[component]
pub fn ThreadPosts(thread: ReadOnlySignal<ThreadModel>) -> Element {
    let posts_result = use_resource(move || get_posts(thread().id)).suspend()?;

    rsx! {
        div { class: "min-w-full border border-gray-400",
            match &*posts_result.read() {
                Ok(post_views) => rsx! {
                    for post_view in post_views {
                        Post { post_view: post_view.clone()}
                    }
                },
                Err(error) => rsx! {
                    ErrorNotice { message: error.to_string() }
                },
            }
        }
    }
}
