use dioxus::prelude::*;

#[component]
pub fn PageContainer(children: Element, extra_classes: Option<String>) -> Element {
    let extra_class_string = extra_classes.unwrap_or("".into());

    rsx! {
        div {class: "h-min-full bg-white dark:bg-slate-950",
            div { class: "container mx-auto px-8 py-4 {extra_class_string}", {children} }
        }
    }
}
