use chrono::NaiveDate;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::env;
use uuid::Uuid;

use crate::routes::*;
pub mod db;
pub mod handlers;

pub mod routes;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub house_number: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LicensePlate {
    pub plate_id: i32,
    pub user_id: Uuid,
    pub license_plate: String,
}

pub struct Card {
    pub id: i32,
    pub user_id: Uuid,
    pub card_id: String,
}

#[derive(Serialize)]
pub struct LicensePlateResponse {
    pub exists: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLicensePlateRequest {
    user_id: Uuid,
    license_plate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCardRequest {
    user_id: Uuid,
    card_id: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LicensePlateRequest {
    pub license_plate: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    serve_routes(pool).await;

    Ok(())
}
