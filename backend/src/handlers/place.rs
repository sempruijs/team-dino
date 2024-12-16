use crate::db::place::*;
use crate::logging::current_time_iso8601;
use crate::types::place::*;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::Reply;

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

pub async fn get_places_handler(pool: PgPool) -> Result<impl Reply, warp::Rejection> {
    match get_places(&pool).await {
        Ok(places) => Ok(warp::reply::with_status(
            warp::reply::json(&places),
            StatusCode::OK,
        )),
        Err(err) => {
            panic!("Database error: {}", err);
        }
    }
}

pub async fn delete_place_handler(
    place_id: Uuid,
    pool: PgPool,
    _uuid: Uuid,
) -> Result<impl Reply, warp::Rejection> {
    match delete_place(&pool, place_id).await {
        Ok(affected_rows) if affected_rows > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "Place deleted successfully"})),
            StatusCode::OK,
        )),
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "Place not found"})),
            StatusCode::NOT_FOUND,
        )),
        Err(err) => {
            panic!("Failed to delete place. Database error: {}", err);
        }
    }
}

#[derive(Deserialize)]

pub struct AvailablePlacesQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub async fn get_available_places_handler(
    query: AvailablePlacesQuery,
    pool: PgPool,
) -> Result<impl Reply, warp::Rejection> {
    match get_available_places(&pool, query.start_date, query.end_date).await {
        Ok(places) => Ok(warp::reply::json(&places)),
        Err(e) => {
            panic!("Error while quering database: {}", e)
        }
    }
}
