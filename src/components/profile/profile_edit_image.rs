use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaUser;
use dioxus_free_icons::Icon;
use supabase_rs::{FileOptions, Supabase};

use base64::{engine::general_purpose, Engine};

use crate::components::general::FileUpload;
use crate::hooks::use_supabase;
use crate::{
    components::general::{ErrorNotice, Snackbar},
    helpers::get_file_data,
};

const AVATAR_BUCKET_ID: &str = "avatars";

async fn upload_file(
    mut supabase: Supabase,
    user_id: String,
    new_profile_image_data: Signal<Option<Vec<u8>>>,
    new_profile_image_type: Signal<Option<mime_guess::Mime>>,
    mut snackbars: Signal<Vec<Element>>,
    mut error_message: Signal<Option<String>>,
) -> () {
    let file_data_and_type =
        new_profile_image_data().and_then(|d| new_profile_image_type().map(|t| (d, t)));
    if let Some((file_data, file_type)) = file_data_and_type {
        let result = supabase
            .upload_file(
                AVATAR_BUCKET_ID,
                file_data,
                format!("{user_id}/avatar").as_str(),
                Some(FileOptions {
                    cache_control: None,
                    content_type: Some(file_type.to_string().as_str()),
                    duplex: None,
                    upsert: true,
                }),
            )
            .await;
        match result {
            Ok(_response) => snackbars.write().push(rsx! {
                Snackbar { message: "Image successfully uploaded." }
            }),
            Err(error) => error_message.set(Some(error.to_string())),
        }
    }
}

#[component]
pub fn ProfileEditImage(user_id: String) -> Element {
    let mut new_profile_image_data = use_signal(|| None);
    let mut new_profile_image_type: Signal<Option<mime_guess::Mime>> = use_signal(|| None);
    let mut error_message = use_signal(|| None);
    let snackbars: Signal<Vec<Element>> = use_signal(|| vec![]);
    let supabase = use_supabase();

    let profile_preview_data = use_memo(move || {
        if let Some(file_data) = new_profile_image_data() {
            Some(general_purpose::STANDARD.encode(file_data))
        } else {
            None
        }
    });

    rsx! {
        div { class: "flex flex-col",
            if let Some(profile_preview_data) = profile_preview_data() {
                img {
                    class: "w-40 h-40 m-auto",
                    src: "data:image/png;base64,{profile_preview_data}",
                }
            } else {
                Icon { class: "w-10 h-10 m-auto", icon: FaUser }
            }
            form {
                class: "space-y-4",
                onsubmit: move |_| upload_file(
                    supabase.clone(),
                    user_id.clone(),
                    new_profile_image_data,
                    new_profile_image_type,
                    snackbars,
                    error_message,
                ),
                FileUpload {
                    accept: "image/*",
                    multiple: false,
                    button_text: "Save Photo",
                    onchange: move |e| async move {
                        let result = get_file_data(
                                e,
                                &mut new_profile_image_data.write(),
                                &mut new_profile_image_type.write(),
                            )
                            .await;
                        if let Err(err) = result {
                            error_message.set(Some(err.to_string()));
                        }
                    }
                }
            }
            if let Some(error_message) = error_message() {
                ErrorNotice { message: error_message }
            }
        }
        for snackbar in snackbars() {
            {snackbar}
        }
    }
}
