use dioxus::prelude::*;

use crate::components::general::BlurredBackdrop;

#[component]
pub fn Dropdown(menu: Element, mut is_active: Signal<bool>) -> Element {
    rsx! {
        BlurredBackdrop { is_active, onclick: move |_| is_active.set(false), z: 20 }
        div { class: "absolute right-0 overflow-auto transform transition-all duration-300 ease-in-out -translate-y-2 translate-y-0 mt-2 z-21",
            if is_active() {
                div {
                    onclick: move |_| is_active.set(false),
                    {menu}
                }
            }
        }
    }
}
