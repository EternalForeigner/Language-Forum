use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TableColumn {
    pub name: String,
    pub extra_classes: Option<String>,
}

#[component]
pub fn ForumTable(
    columns: Vec<TableColumn>,
    rows: Vec<Element>,
    classes: Option<String>,
) -> Element {
    rsx! {
        table { class: classes,
            thead {
                tr {
                    for column in columns {
                        th {
                            class: String::from("px-6 py-3 text-left text-xs font-medium uppercase tracking-wider ")
                                + &column.extra_classes.unwrap_or_default(),
                            {column.name.clone()}
                        }
                    }
                }
            }
            tbody {
                for row in rows {
                    {row}
                }
            }
        }
    }
}
