use dioxus::prelude::*;

use crate::{models::Category, Route};

#[component]
pub fn CategoryRow(category: Category, even: bool) -> Element {
    let row_color = if even { "bg-gray-900" } else { "bg-gray-800" };

    rsx! {
        Link {
            to: Route::Category { category_id: category.id },
            tr { class: "bg-gray-800 hover:bg-gray-700 transition-colors flex basis-full {row_color}",
                td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-100",
                    {category.name}
                }
            }
        }
    }
}
