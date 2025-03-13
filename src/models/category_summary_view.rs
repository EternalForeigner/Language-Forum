#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct CategorySummaryView {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub required_role: Option<i16>,
    pub creator_id: Option<uuid::Uuid>,
    pub index: Option<i16>,
    pub thread_count: Option<i64>,
    pub post_count: Option<i64>,
    pub latest_post_id: Option<i64>,
    pub latest_post_date: Option<chrono::DateTime<chrono::Utc>>,
    pub latest_post_creator_name: Option<String>,
}