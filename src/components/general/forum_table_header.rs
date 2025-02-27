use dioxus::prelude::*;

const HEADER_CLASSES: &str =
    "px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider";

#[component]
pub fn ForumTableHeader(titles: Vec<String>) -> Element {
    rsx! {
        thead {
            tr { class: "flex flex-row",
                for (index , title) in titles.iter().enumerate() {
                    {
                        let classes = format!{"{HEADER_CLASSES} {}", if index == 0 {"basis-3/4"} else {""}};
                        rsx! {
                            th { class: classes, {title.clone()} }
                        }
                    }
                }
            }
        }
    }
}
