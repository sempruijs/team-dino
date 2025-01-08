use crate::db::card::*;
use crate::types::card::*;
use serde_json::json;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;

pub async fn check_card_handler(
    card_id: String,
    pool: PgPool,
    _user_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    match check_card(&pool, &card_id).await {
        Ok(exists) => {
            if exists {
                Ok(warp::reply::json(&true))
            } else {
                Ok(warp::reply::json(&false))
            }
        }
        Err(e) => {
            panic!("Error checking card: {}", e);
        }
    }
}

pub async fn create_card_handler(
    request: CreateCardRequest,
    pool: PgPool,
    _user_id: Uuid,
) -> Result<impl Reply, Rejection> {
    match create_card(&pool, request.user_id, request.card_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "Card created successfully"})),
            StatusCode::CREATED,
        )),
        Err(err) => {
            eprintln!("Database error: {}", err);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "Failed to create card"})),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
