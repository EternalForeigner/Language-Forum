use dioxus::prelude::*;
use supabase_rs::Result;

use crate::{
    components::{
        forum::ThreadRow,
        general::{ErrorNotice, ForumTable, ForumTableHeader},
    },
    hooks::use_supabase,
    models::Thread,
};

const HEADERS: [&str; 2] = ["Title", "Last Reply"];

async fn get_threads_by_category(category_id: i64) -> Result<Vec<Thread>> {
    let response = use_supabase()
        .from("threads")
        .await?
        .eq("category_id", category_id.to_string())
        // TODO: order by most recent post
        .execute()
        .await?
        .error_for_status()?;

    Ok(response.json::<Vec<Thread>>().await?)
}

#[component]
pub fn ThreadsTable(category_id: ReadOnlySignal<i64>) -> Element {
    let threads_result = use_resource(move || get_threads_by_category(category_id())).suspend()?;

    rsx! {
        match &*threads_result.read() {
            Ok(threads) => rsx! {
                ForumTable {
                    extra_classes: "border border-gray-400",
                    head: rsx! {
                        ForumTableHeader { titles: HEADERS.iter().map(|string| String::from(*string)).collect() }
                    },
                    rows: threads
                        .iter()
                        .enumerate()
                        .map(|(index, thread)| rsx! {
                            ThreadRow { thread: thread.clone(), even: index % 2 == 0 }
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
