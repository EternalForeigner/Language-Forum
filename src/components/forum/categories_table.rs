use dioxus::prelude::*;
use supabase_rs::Result;

use crate::{
    components::{
        forum::CategoryRow,
        general::{ErrorNotice, ForumTable, TableColumn},
    },
    hooks::use_supabase,
    models::CategorySummaryView,
};

async fn get_categories(parent_id: Option<i64>) -> Result<Vec<CategorySummaryView>> {
    let mut builder = use_supabase().from("category_summary_view").await?;

    builder = if let Some(parent_id) = parent_id {
        builder.eq("parent_id", parent_id.to_string())
    } else {
        builder.is("parent_id", "null")
    };

    let response = builder
        .order("index.asc")
        .execute()
        .await?
        .error_for_status()?;

    Ok(response.json::<Vec<CategorySummaryView>>().await?)
}

#[component]
pub fn CategoriesTable(parent_id: Option<i64>) -> Element {
    let category_summaries_result =
        use_resource(use_reactive!(|(parent_id,)| get_categories(parent_id))).suspend()?;

    let header_classes = "bg-blue-100 dark:bg-slate-800 text-gray-800 dark:text-gray-400";

    rsx! {
        match &*category_summaries_result.read() {
            Ok(category_summaries) => rsx! {
                if category_summaries.len() > 0 {
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
                        rows: category_summaries
                            .iter()
                            .enumerate()
                            .map(|(index, category_summary)| rsx! {
                                CategoryRow { category_summary: category_summary.clone(), even: index % 2 == 0 }
                            })
                            .collect(),
                    }
                }
            },
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        }
    }
}
