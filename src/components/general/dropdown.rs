use dioxus::prelude::*;

#[component]
pub fn Dropdown(is_open: bool, menu: Element) -> Element {

    // TODO: https://github.com/samtay/birdtalk/blob/5a8b1ec0263b81619f591e00afa1edc99ec8cf74/app/src/ui/components/modal.rs
    rsx! {
        if is_open {
            div { class: "absolute right-0 overflow-auto transform transition-all duration-300 ease-in-out -translate-y-2 translate-y-0 mt-2",
                {menu}
            }
        }
    }
}
