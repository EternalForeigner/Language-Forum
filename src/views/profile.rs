use dioxus::prelude::*;
use uuid::Uuid;
use supabase_rs::{Result, Session};

use crate::components::general::ErrorNotice;
use crate::components::profile::{ProfileEditForm, ProfileEditImage};
use crate::hooks::use_supabase;
use crate::models::Profile as ProfileModel;

async fn get_profile(user_id: Uuid) -> Result<Option<ProfileModel>> {
    let response = use_supabase()
        .from("profiles")
        .await?
        .eq("id", user_id.to_string())
        .execute()
        .await?
        .error_for_status()?;

    let list = response.json::<Vec<ProfileModel>>().await?;
    Ok(list.first().cloned())
}

fn new_profile(user_id: Uuid) -> ProfileModel {
    ProfileModel {
        id: user_id,
        name: String::new(),
        updated_at: chrono::Utc::now(),
    }
}

#[component]
pub fn Profile() -> Element {
    let user = use_context::<Session>().user;

    let profile_user_id = user.id.clone();
    let profile = use_resource(move || {
        let user_id = profile_user_id.clone();
        async move { get_profile(user_id).await }
    });

    let content = match &*profile.read() {
        Some(response) => match response {
            Ok(profile) => rsx! {
                div { class: "flex flex-col space-y-4",
                    ProfileEditImage { user_id: user.id }
                    ProfileEditForm { current_profile: profile.clone().unwrap_or(new_profile(user.id)) }
                }
            },
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        },
        None => rsx! {
            div { "loading" }
        },
    };

    rsx! {
        div { class: "container mx-auto py-4 px-8",
            div { class: "max-w-lg mx-auto", {content} }
        }
    }
}
