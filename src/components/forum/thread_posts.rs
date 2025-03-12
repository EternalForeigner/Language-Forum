use dioxus::{logger::tracing, prelude::*};
use supabase_rs::Result;
use uuid::Uuid;

use crate::{
    components::{forum::Post, general::ErrorNotice},
    hooks::use_supabase,
    models::{PostView, Thread as ThreadModel},
};

async fn get_posts(thread_id: Uuid) -> Result<Vec<PostView>> {
    tracing::debug!("getting posts");
    let response = use_supabase()
        .from("post_view")
        .await?
        .eq("thread_id", thread_id.to_string())
        .order("created_at.asc")
        .execute()
        .await?
        .error_for_status()?;

    tracing::debug!("got posts");
    Ok(response.json::<Vec<PostView>>().await?)
}

#[component]
pub fn ThreadPosts(key_: ReadOnlySignal<i32>, thread: ReadOnlySignal<ThreadModel>) -> Element {
    let mut posts_resource = use_resource(move || get_posts(thread().id));

    // workaround for key prop not properly triggering a rerender
    use_effect(move || {
        key_();
        posts_resource.restart();
    });

    let post_views = posts_resource.suspend()?;

    rsx! {
        div { class: "min-w-full border border-gray-400",
            match &*post_views.read() {
                Ok(post_views) => rsx! {
                    for post_view in post_views {
                        Post { post_view: post_view.clone() }
                    }
                },
                Err(error) => rsx! {
                    ErrorNotice { message: error.to_string() }
                },
            }
        }
    }
}
