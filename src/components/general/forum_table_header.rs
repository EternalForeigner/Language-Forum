use dioxus::prelude::*;

#[component]
pub fn ForumTableHeader(titles: Vec<String>, extra_classes: Option<String>) -> Element {
    rsx! {
        thead {
            tr {
                for (index , title) in titles.iter().enumerate() {
                    {
                        let classes = format!(
                            "px-6 py-3 text-left text-xs font-medium uppercase tracking-wider {} {}",
                            if index == 0 { "w-auto" } else { "" },
                            extra_classes.clone().unwrap_or_default(),
                        );
                        rsx! {
                            th { class: classes, {title.clone()} }
                        }
                    }
                }
            }
        }
    }
}
