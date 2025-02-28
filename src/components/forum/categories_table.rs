use dioxus::prelude::*;
use supabase_rs::Result;

use crate::{
    components::{
        forum::CategoryRow,
        general::{ErrorNotice, ForumTable, ForumTableHeader},
    },
    hooks::use_supabase,
    models::Category,
};

const HEADERS: [&str; 2] = ["Title", "Last Post"];

async fn get_categories(parent_id: Option<i64>) -> Result<Vec<Category>> {
    let mut builder = use_supabase().from("categories").await?;

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

    Ok(response.json::<Vec<Category>>().await?)
}

#[component]
pub fn CategoriesTable(parent_id: Option<i64>) -> Element {
    let categories_result =
        use_resource(use_reactive!(|(parent_id,)| get_categories(parent_id))).suspend()?;

    rsx! {
        match &*categories_result.read() {
            Ok(categories) => rsx! {
                if categories.len() > 0 {
                    ForumTable {
                        extra_classes: "border border-white",
                        head: rsx! {
                            ForumTableHeader { titles: HEADERS.iter().map(|string| String::from(*string)).collect() }
                        },
                        rows: categories
                            .iter()
                            .enumerate()
                            .map(|(index, category)| rsx! {
                                CategoryRow { category: category.clone(), even: index % 2 == 0 }
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
