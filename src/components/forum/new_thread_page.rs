use dioxus::prelude::*;
use supabase_rs::Result;

use crate::{
    components::{
        forum::{CategoriesTable, ThreadsTable},
        general::ErrorNotice,
    },
    hooks::use_supabase,
    models::Category as CategoryModel,
};

async fn get_category(id: i64) -> Result<Option<CategoryModel>> {
    let response = use_supabase()
        .from("categories")
        .await?
        .eq("id", id.to_string())
        .execute()
        .await?
        .error_for_status()?;

    let list = response.json::<Vec<CategoryModel>>().await?;
    Ok(list.first().cloned())
}

#[component]
pub fn NewThreadPage(category_id: i64) -> Element {
    let category = use_resource(move || get_category(category_id)).suspend()?;

    rsx! {
        match &*category.read() {
            Ok(category) => {
                if let Some(category) = category {
                    rsx! {
                        p { class: "text-sm text-gray-300", {format!("Posting to: {}", category.name.clone())} }
                        div { class: "my-8" }
                    }
                } else {
                    rsx! {
                        ErrorNotice { message: "Resource not found! Please check the URL." }
                    }
                }
            }
            Err(error) => rsx! {
                ErrorNotice { message: error.to_string() }
            },
        }
    }
}
