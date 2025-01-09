use crate::controller::routes::serve_routes;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Import different modules that make up the backend
pub mod controller;
pub mod repository;
pub mod service;
pub mod traits;

// types that are used accross the modules.
pub mod types;

// Main gets executed on startup.
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Read the database path from enviousment variables.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("DINO_SECRET_KEY").expect("DINO_SECRET_KEY must be set");

    // thread pool that executes sql queries.
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // Expose all api's
    serve_routes(pool, secret_key).await;

    Ok(())
}
