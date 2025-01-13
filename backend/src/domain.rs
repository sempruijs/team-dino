use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Place {
    pub place_id: Uuid,
    pub house_number: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub place_id: Uuid,
}
