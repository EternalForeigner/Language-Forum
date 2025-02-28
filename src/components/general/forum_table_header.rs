use dioxus::prelude::*;

#[component]
pub fn ForumTableHeader(titles: Vec<String>) -> Element {
    rsx! {
        thead {
            tr {
                for (index , title) in titles.iter().enumerate() {
                    {
                        let classes = format!(
                            "px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider {}",
                            if index == 0 { "w-auto" } else { "" },
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
