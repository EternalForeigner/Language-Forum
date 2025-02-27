use dioxus::prelude::*;

use crate::components::forum::CategoriesTable;

#[component]
pub fn Category(category_id: i64) -> Element {
    rsx! {
        div { class: "container mx-auto py-4 px-8",
            CategoriesTable { parent_id: category_id }
        }
    }
}
