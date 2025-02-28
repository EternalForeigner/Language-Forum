#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,PartialEq)]
pub struct PostView {
    pub post_id: Option<i64>,
    pub text_body: Option<String>,
    pub thread_id: Option<uuid::Uuid>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub profile_id: Option<uuid::Uuid>,
    pub profile_name: Option<String>,
}