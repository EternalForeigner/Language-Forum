use dioxus::prelude::*;

#[component]
pub fn ImageOrFallback(
    image_url: Option<String>,
    image_class: String,
    fallback: Element,
) -> Element {
    let mut image_exists = use_signal(|| true);

    if let Some(image_url) = image_url {
        if image_exists() {
            return rsx! {
                img {
                    class: image_class,
                    src: image_url,
                    onerror: move |_| image_exists.set(false),
                }
            };
        }
    }

    fallback
}
