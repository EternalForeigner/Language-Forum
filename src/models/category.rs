#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct Category {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub required_role: i16,
    pub creator_id: uuid::Uuid,
}