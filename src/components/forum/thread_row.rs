use dioxus::prelude::*;

use crate::{models::ThreadSummaryView, Route};

#[component]
pub fn ThreadRow(thread_summary: ThreadSummaryView, even: bool) -> Element {
    let row_color = if even {
        "bg-white dark:bg-slate-900"
    } else {
        "bg-slate-50 dark:bg-slate-800"
    };

    rsx! {
        tr { class: "hover:bg-cyan-50 dark:hover:bg-gray-700 transition-colors {row_color}",
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                if let Some(id) = thread_summary.id {
                    Link {
                        class: "block size-full px-6 py-4",
                        to: Route::Thread { thread_id: id },
                        {thread_summary.title.unwrap_or(String::from("[Unknown thread]"))}
                    }
                }
            }
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                if let Some(reply_count) = thread_summary.post_count {
                    div { class: "h-full px-6 py-4", "Replies: {reply_count}" }
                }
            }
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                div { class: "h-full px-6 py-4",
                    {
                        String::from("Last post by: ")
                            + &thread_summary
                                .latest_post_creator_name
                                .unwrap_or(String::from("[Unknown user]"))
                    }
                    br {}
                    {
                        String::from("On: ")
                            + &thread_summary
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
