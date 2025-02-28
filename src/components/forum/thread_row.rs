use dioxus::prelude::*;

use crate::{models::Thread, Route};

#[component]
pub fn ThreadRow(thread: Thread, even: bool) -> Element {
    let row_color = if even { "bg-gray-900" } else { "bg-gray-800" };

    rsx! {
        tr { class: "w-auto bg-gray-800 hover:bg-gray-700 transition-colors {row_color}",
            td { class: "whitespace-nowrap text-sm font-medium text-gray-100",
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
