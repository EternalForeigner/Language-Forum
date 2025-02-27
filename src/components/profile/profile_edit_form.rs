use dioxus::prelude::*;
use supabase_rs::{Result, SupabaseError};

use crate::components::general::{ErrorNotice, Snackbar};
use crate::components::{BUTTON_CLASSES, INPUT_CLASSES};
use crate::hooks::use_supabase;
use crate::models::Profile as ProfileModel;

async fn upsert_profile(profile: &ProfileModel) -> Result<()> {
    let json = serde_json::to_string(profile).map_err(|e| SupabaseError::Unknown(Box::new(e)))?;
    use_supabase()
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
    let mut error_message = use_signal(|| None);
    let mut snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);

    rsx! {
        form {
            class: "flex flex-col space-y-4",
            onsubmit: move |_| async move {
                match upsert_profile(&new_profile()).await {
                    Ok(_) => snackbars.write().push(rsx! {
                        Snackbar { message: "Profile updated." }
                    }),
                    Err(error) => error_message.set(Some(error.to_string())),
                }
            },
            label { r#for: "display_name", "Display Name" }
            input {
                name: "display_name",
                placeholder: "Enter your display name",
                r#type: "text",
                class: "{INPUT_CLASSES}",
                id: "display_name",
                value: new_profile().name,
                oninput: move |e| new_profile.write().name = e.value(),
            }
            button { r#type: "submit", class: "{BUTTON_CLASSES}", "Save" }
            if let Some(error_message) = error_message() {
                ErrorNotice { message: error_message }
            }
        }
        for snackbar in snackbars() {
            {snackbar}
        }
    }
}
