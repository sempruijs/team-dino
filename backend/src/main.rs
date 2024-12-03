use crate::routes::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Import different modules that make up the backend
// db for reading and writing into the database
pub mod db;

// handlers wrap the database functions so that it has rest api like outputs.
pub mod handlers;

// For hashing information
pub mod hash;

// For printing information
pub mod logging;

// Routes makes from the handlers rest api endpoints
pub mod routes;

// types that are used accross the modules.
pub mod types;

// Main gets executed on startup.
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Read the database path from enviousment variables.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // thread pool that executes sql queries.
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // Expose all api's
    serve_routes(pool).await;

    Ok(())
}
