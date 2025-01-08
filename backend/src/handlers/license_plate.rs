use crate::db::license_plate::*;
use crate::types::license_plate::*;
use serde_json::json;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;

pub async fn check_license_plate_handler(
    plate: String, // Assuming you have a struct LicensePlateRequest for deserialization
    pool: PgPool,
    _user_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    match license_plate_is_valid(&pool, &plate).await {
        Ok(valid) => Ok(warp::reply::json(&valid)),
        Err(e) => {
            panic!("Error checking license plate: {}", e);
        }
    }
}

pub async fn create_license_plate_handler(
    request: CreateLicensePlateRequest,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    match create_license_plate(&pool, request.user_id, request.license_plate).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "License plate created successfully"})),
            StatusCode::CREATED,
        )),
        Err(err) => {
            eprintln!("Database error: {}", err);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "Failed to create license plate"})),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
