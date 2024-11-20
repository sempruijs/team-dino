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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    serve_routes(pool).await;

    Ok(())
}
