#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct Post {
    pub id: i64,
    pub thread_id: uuid::Uuid,
    pub creator_id: uuid::Uuid,
    pub text_body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}