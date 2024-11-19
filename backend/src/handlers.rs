use crate::db::*;
use crate::CreateLicensePlateRequest;
use crate::LicensePlateRequest;
use crate::{Ticket, User};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;

fn current_time_iso8601() -> String {
    Utc::now().to_rfc3339()
}

pub async fn create_user_handler(
    user: User,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601();
    println!("New user created: {:?}  ({})", user, now);

    match create_user(&pool, user).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => panic!("error while listing users"),
    }
}

pub async fn create_ticket_handler(
    ticket: Ticket,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601(); // Assuming this function returns the current time as ISO8601
    println!(
        "New ticket created for user_id: {}  ({})",
        ticket.user_id, now
    );

    match create_ticket(&pool, ticket).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => {
            panic!("Error while creating ticket");
        }
    }
}

pub async fn check_license_plate_handler(
    plate: String, // Assuming you have a struct LicensePlateRequest for deserialization
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match check_license_plate(&pool, &plate).await {
        Ok(exists) => {
            if exists {
                Ok(warp::reply::json(&true)) // If plate exists, return true
            } else {
                Ok(warp::reply::json(&false)) // If plate does not exist, return false
            }
        }
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
