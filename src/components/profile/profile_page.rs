use dioxus::prelude::*;
use supabase_rs::{Result, Session, Supabase};
use uuid::Uuid;

use crate::components::general::ErrorNotice;
use crate::components::profile::{ProfileEditForm, ProfileEditImage};
use crate::hooks::use_supabase;
use crate::models::Profile as ProfileModel;

async fn get_profile(mut supabase: Supabase, user_id: Uuid) -> Result<Option<ProfileModel>> {
    let response = supabase
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
pub fn ProfilePage() -> Element {
    let user = use_context::<Session>().user;
    let supabase = use_supabase();

    let profile_user_id = user.id.clone();
    let profile = use_resource(move || {
        let supabase_clone = supabase.clone();
        let user_id = profile_user_id.clone();
        async move { get_profile(supabase_clone, user_id).await }
    })
    .suspend()?;

    rsx! {
        match &*profile.read() {
            Ok(profile) => rsx! {
                div { class: "flex flex-col space-y-4",
                    ProfileEditImage { user_id: user.id }
                    ProfileEditForm { current_profile: profile.clone().unwrap_or(new_profile(user.id)) }
                }
            },
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        }
    }
}
