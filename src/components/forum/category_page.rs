use dioxus::prelude::*;
use supabase_rs::{Result, Supabase};

use crate::{
    components::{
        forum::{CategoriesTable, ThreadsTable},
        general::ErrorNotice,
        BUTTON_CLASSES,
    },
    hooks::use_supabase,
    models::CategorySummaryView,
    Route,
};

async fn get_category(mut supabase: Supabase, id: i64) -> Result<Option<CategorySummaryView>> {
    let response = supabase
        .from("category_summary_view")
        .await?
        .eq("id", id.to_string())
        .execute()
        .await?
        .error_for_status()?;

    let list = response.json::<Vec<CategorySummaryView>>().await?;
    Ok(list.first().cloned())
}

#[component]
pub fn CategoryPage(category_id: ReadOnlySignal<i64>) -> Element {
    let supabase = use_supabase();
    let is_logged_in = supabase.is_logged_in();
    let category_summary =
        use_resource(move || get_category(supabase.clone(), category_id())).suspend()?;

    rsx! {
        match &*category_summary.read() {
            Ok(category_summary) => {
                if let Some(category_summary) = category_summary {
                    rsx! {
                        h1 { class: "my-2 text-3xl text-gray-950 dark:text-white", {category_summary.name.clone()} }
                        p { class: "text-sm text-gray-700 dark:text-gray-300", {category_summary.description.clone()} }
                        div { class: "flex my-4 justify-end",
                            if is_logged_in {
                                Link { to: Route::CreateThread { category_id: category_id() },
                                    button { class: BUTTON_CLASSES, "New Thread" }
                                }
                            }
                        }
                        CategoriesTable { parent_id: category_id() }
                        ThreadsTable { category_id: category_id() }
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
