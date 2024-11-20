use crate::routes::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub mod db;
pub mod handlers;
pub mod logging;
pub mod routes;
pub mod types;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    serve_routes(pool).await;

    Ok(())
}
