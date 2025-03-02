use dioxus::prelude::*;

#[component]
pub fn BlurredBackdrop(is_active: ReadOnlySignal<bool>, onclick: Callback<Event<MouseData>>, z: i32) -> Element {
    rsx! {
        div {
            class: if is_active() { "fixed inset-0 z-{z} flex justify-center items-center backdrop-blur-sm bg-offwhite-1/20 overflow-hidden transition-[opacity,backdrop-blur] duration-500" } else { "invisible z-[-1] opacity-0 backdrop-blur-none" },
            onclick,
        }
    }
}
