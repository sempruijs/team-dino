use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

pub struct Card {
    pub id: i32,
    pub user_id: Uuid,
    pub card_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCardRequest {
    pub user_id: Uuid,
    pub card_id: String,
}
