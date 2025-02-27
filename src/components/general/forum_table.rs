use dioxus::prelude::*;

#[component]
pub fn ForumTable(head: Element, rows: Vec<Element>, extra_classes: Option<String>) -> Element {
    let classes = format! {"min-w-full {}", extra_classes.unwrap_or(String::new())};

    rsx! {
        table {
            class: classes,
            {head}
            tbody {
                for row in rows {
                    {row}
                }
            }
        }
    }
}
