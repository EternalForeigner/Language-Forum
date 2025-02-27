#![allow(dead_code)]
// Generated with autostruct
// https://github.com/sound-systems/autostruct

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Profile {
    pub id: uuid::Uuid,
    pub name: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
