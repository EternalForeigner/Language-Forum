use dioxus::prelude::*;
use supabase_rs::{Result, Supabase, SupabaseError};

use crate::{
    components::{
        general::{ErrorNotice, Snackbar, SubmitButton},
        INPUT_CLASSES, LABEL_CLASSES,
    },
    hooks::use_supabase,
    models::Profile as ProfileModel,
};

async fn upsert_profile(mut supabase: Supabase, profile: &ProfileModel) -> Result<()> {
    let json = serde_json::to_string(profile).map_err(|e| SupabaseError::Unknown(Box::new(e)))?;
    supabase
        .from("profiles")
        .await?
        .upsert(json)
        .execute()
        .await?;

    Ok(())
}

#[component]
pub fn ProfileEditForm(current_profile: ProfileModel) -> Element {
    let mut new_profile = use_signal(|| current_profile);
    let mut is_submitting = use_signal(|| false);
    let mut error_message = use_signal(|| None);
    let mut snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);
    let supabase = use_supabase();

    rsx! {
        form {
            class: "flex flex-col space-y-4",
            onsubmit: move |_| {
                is_submitting.set(true);
                let supabase_clone = supabase.clone();
                async move {
                    match upsert_profile(supabase_clone, &new_profile()).await {
                        Ok(_) => snackbars.write().push(rsx! {
                            Snackbar { message: "Profile updated." }
                        }),
                        Err(error) => error_message.set(Some(error.to_string())),
                    }
                    is_submitting.set(false);
                }
            },
            label { class: LABEL_CLASSES, r#for: "display_name", "Display Name" }
            input {
                name: "display_name",
                placeholder: "Enter your display name",
                r#type: "text",
                class: INPUT_CLASSES,
                id: "display_name",
                value: new_profile().name,
                oninput: move |e| new_profile.write().name = e.value(),
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
