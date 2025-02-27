use dioxus::prelude::*;

const HEADER_CLASSES: &str =
    "px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider";

#[component]
pub fn ForumTableHeader(titles: Vec<String>) -> Element {
    rsx! {
        thead {
            tr {
                for (index , title) in titles.iter().enumerate() {
                    {
                        let classes = format!{"{HEADER_CLASSES} {}", if index == 0 {"w-auto"} else {""}};
                        rsx! {
                            th { class: classes, {title.clone()} }
                        }
                    }
                }
            }
        }
    }
}
