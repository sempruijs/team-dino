use crate::db::place::*;
use crate::logging::current_time_iso8601;
use crate::types::place::*;
use sqlx::PgPool;
use warp::http::StatusCode;

pub async fn create_place_handler(
    place: Place,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601(); // Assuming this function returns the current time as ISO8601

    match create_place(&pool, place.clone()).await {
        Ok(_) => {
            println!("New place created: {} at: {}", place.place_id, now);
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            panic!("Error while creating place: {}", e);
        }
    }
}
