use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::FromRow;
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
}
