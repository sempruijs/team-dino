use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub password: String,
}
