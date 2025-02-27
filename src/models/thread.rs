#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct Thread {
    pub id: uuid::Uuid,
    pub creator_id: uuid::Uuid,
    pub title: String,
    pub category_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub post_required_rank: i16,
}