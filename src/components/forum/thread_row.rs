use dioxus::prelude::*;

use crate::{models::Thread, Route};

#[component]
pub fn ThreadRow(thread: Thread, even: bool) -> Element {
    let row_color = if even { "bg-white dark:bg-slate-900" } else { "bg-slate-50 dark:bg-slate-800" };

    rsx! {
        tr { class: "w-auto hover:bg-cyan-50 dark:hover:bg-gray-700 transition-colors {row_color}",
            td { class: "whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-100",
                Link {
                    class: "block h-full w-full px-6 py-4",
                    to: Route::Thread {
                        thread_id: thread.id,
                    },
                    {thread.title}
                }
            }
        }
    }
}
