use dioxus::prelude::*;

#[component]
pub fn Dropdown(is_open: bool, menu: Element) -> Element {
    rsx! {
        if is_open {
            div { class: "absolute right-0 overflow-auto transform transition-all duration-300 ease-in-out -translate-y-2 translate-y-0 mt-2",
                {menu}
            }
        }
    }
}
