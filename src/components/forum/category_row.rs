use dioxus::prelude::*;

use crate::{models::CategorySummaryView, Route};

// TODO: consolidate this with ThreadRow
#[component]
pub fn CategoryRow(category_summary: CategorySummaryView, even: bool) -> Element {
    let row_color = if even {
        "bg-white dark:bg-slate-900"
    } else {
        "bg-slate-50 dark:bg-slate-800"
    };

    rsx! {
        tr { class: "hover:bg-cyan-50 dark:hover:bg-gray-700 transition-colors {row_color}",
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                if let Some(id) = category_summary.id {
                    Link {
                        class: "block h-full px-6 py-4",
                        to: Route::Category { category_id: id },
                        {category_summary.name}
                    }
                }
            }
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                div { class: "h-full px-6 py-4",
                    if let Some(thread_count) = category_summary.thread_count {
                        "Threads: {thread_count}"
                    }
                    br {}
                    if let Some(reply_count) = category_summary.post_count {
                        "Replies: {reply_count}"
                    }
                }
            }
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                div { class: "h-full px-6 py-4",
                    {
                        String::from("Last post by: ")
                            + &category_summary
                                .latest_post_creator_name
                                .unwrap_or(String::from("[Unknown user]"))
                    }
                    br {}
                    {
                        String::from("On: ")
                            + &category_summary
                                .latest_post_date
                                .map_or(
                                    String::from("[Unknown date]"),
                                    |d| d.format("%B %d, %Y %H:%M").to_string(),
                                )
                    }
                }
            }
        }
    }
}
