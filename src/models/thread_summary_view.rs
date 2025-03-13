#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct ThreadSummaryView {
    pub id: Option<uuid::Uuid>,
    pub creator_id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category_id: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub post_required_rank: Option<i16>,
    pub post_count: Option<i64>,
    pub latest_post_id: Option<i64>,
    pub latest_post_date: Option<chrono::DateTime<chrono::Utc>>,
    pub latest_post_creator_name: Option<String>,
}