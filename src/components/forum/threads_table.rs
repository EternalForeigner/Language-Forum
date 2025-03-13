use dioxus::prelude::*;
use supabase_rs::Result;

use crate::{
    components::{
        forum::ThreadRow,
        general::{ErrorNotice, ForumTable, TableColumn},
    },
    hooks::use_supabase,
    models::ThreadSummaryView,
};

async fn get_threads_by_category(category_id: i64) -> Result<Vec<ThreadSummaryView>> {
    let response = use_supabase()
        .from("thread_summary_view")
        .await?
        .eq("category_id", category_id.to_string())
        .order("latest_post_date.desc")
        .execute()
        .await?
        .error_for_status()?;

    Ok(response.json::<Vec<ThreadSummaryView>>().await?)
}

#[component]
pub fn ThreadsTable(category_id: ReadOnlySignal<i64>) -> Element {
    let threads_result = use_resource(move || get_threads_by_category(category_id())).suspend()?;

    let header_classes = "bg-blue-100 dark:bg-slate-800 text-gray-800 dark:text-gray-400";

    rsx! {
        match &*threads_result.read() {
            Ok(thread_summary) => rsx! {
                ForumTable {
                    classes: "border border-gray-400",
                    columns: vec![
                        TableColumn {
                            name: String::from("Title"),
                            extra_classes: Some(String::from(header_classes) + " min-w-full"),
                        },
                        TableColumn {
                            name: String::from("Information"),
                            extra_classes: Some(String::from(header_classes)),
                        },
                        TableColumn {
                            name: String::from("Last Post"),
                            extra_classes: Some(String::from(header_classes)),
                        },
                    ],
                    rows: thread_summary
                        .iter()
                        .enumerate()
                        .map(|(index, thread_summary)| rsx! {
                            ThreadRow { thread_summary: thread_summary.clone(), even: index % 2 == 0 }
                        })
                        .collect(),
                }
            },
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        }
    }
}
