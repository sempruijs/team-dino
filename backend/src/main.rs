use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::routes::*;
pub mod db;
pub mod handlers;

pub mod routes;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,           // Primary key for the user
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub license_plate: Vec<String>, // Store as JSONB in PostgreSQL
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub ticket_id: Uuid,          // Primary key for the ticket
    pub user_id: Uuid,            // Foreign key referencing the user
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub house_number: u32,
}

#[derive(Serialize)]
pub struct LicensePlateResponse {
    pub exists: bool,
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
