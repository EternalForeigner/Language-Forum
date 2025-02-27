use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaUser;
use dioxus_free_icons::Icon;
use supabase_rs::{FileOptions, Supabase};

use base64::{engine::general_purpose, Engine};

use crate::{components::BUTTON_CLASSES, helpers::get_file_data};

const AVATAR_BUCKET_ID: &str = "avatars";

#[component]
pub fn ProfileEditImage(user_id: String) -> Element {
    let mut new_profile_image_data = use_signal(|| None);
    let mut new_profile_image_type: Signal<Option<mime_guess::Mime>> = use_signal(|| None);
    let mut error = use_signal(|| None);

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
                class: "flex flex-col space-y-4",
                onsubmit: move |_| {
                    let user_id_clone = user_id.clone();
                    async move {
                        let file_data_and_type = new_profile_image_data()
                            .and_then(|d| new_profile_image_type().map(|t| (d, t)));
                        if let Some((file_data, file_type)) = file_data_and_type {
                            let mut supabase = consume_context::<Supabase>();
                            let _ = supabase
                                .upload_file(
                                    AVATAR_BUCKET_ID,
                                    file_data,
                                    user_id_clone.as_str(),
                                    Some(FileOptions {
                                        cache_control: None,
                                        content_type: Some(file_type.to_string().as_str()),
                                        duplex: None,
                                        upsert: true,
                                    }),
                                )
                                .await;
                        }
                    }
                },
                input {
                    id: "select-file",
                    name: "select-file",
                    r#type: "file",
                    accept: ".jpg .jpeg .png .bmp .gif webp",
                    multiple: false,
                    onchange: move |e| async move {
                        let result = get_file_data(
                                e,
                                &mut new_profile_image_data.write(),
                                &mut new_profile_image_type.write(),
                            )
                            .await;
                        if let Err(err) = result {
                            error.set(Some(err));
                        }
                    },
                }
                button { class: BUTTON_CLASSES, r#type: "submit", "Upload photo" }
            }
        }
    }
}
