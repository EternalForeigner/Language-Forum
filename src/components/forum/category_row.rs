use dioxus::prelude::*;

use crate::{models::Category, Route};

// TODO: consolidate this with ThreadRow
#[component]
pub fn CategoryRow(category: Category, even: bool) -> Element {
    let row_color = if even { "bg-white dark:bg-slate-900" } else { "bg-slate-50 dark:bg-slate-800" };

    rsx! {
        tr { class: "hover:bg-cyan-50 dark:hover:bg-gray-700 transition-colors {row_color}",
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                Link {
                    class: "block h-full px-6 py-4",
                    to: Route::Category {
                        category_id: category.id,
                    },
                    {category.name}
                }
            }
        }
    }
}
