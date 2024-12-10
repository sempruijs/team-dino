use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Place {
    pub place_id: Uuid,
    pub house_number: i32,
}
