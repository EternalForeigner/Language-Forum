use dioxus::prelude::*;

use crate::components::{general::LoadingIndicator, BUTTON_CLASSES};

#[component]
pub fn SubmitButton(
    is_loading: bool,
    r#type: Option<String>,
    extra_classes: Option<String>,
    children: Element,
) -> Element {
    let r#type = r#type.unwrap_or("submit".into());
    let extra_classes = extra_classes.unwrap_or_default();

    rsx! {
        button { class: "{BUTTON_CLASSES} {extra_classes}", r#type,
            div { class: "flex gap-x-2 justify-center",
                if is_loading {
                    LoadingIndicator { extra_classes: "size-5 text-white" }
                }
                {children}
            }
        }
    }
}
