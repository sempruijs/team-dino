use crate::service::logging::*;
use crate::service::user::*;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;
use utoipa::{OpenApi, ToSchema};
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply; // Assuming PgPool is defined elsewhere

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub date_of_birth: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Bad request")
    ),
    operation_id = "create_user",
    tag = "Users"
)]
pub async fn create_user_handler(
    request: web::Json<CreateUser>, // Automatically deserializes the request body into `CreateUser`
    pool: web::Data<PgPool>,        // PgPool is shared across requests
) -> impl Responder {
    let now = current_time_iso8601(); // Assuming this function exists
    println!("Creating new user, {}", now);

    // Try to create the user with the provided pool and data
    match create_user(&pool, request.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(), // Return 201 Created
        Err(e) => {
            // Handle the error (you can log or return more detailed error messages)
            println!("Error while creating user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e)) // Return 500 Internal Server Error
        }
    }
}

// pub async fn get_user_handler(pool: PgPool, user_id: Uuid) -> Result<impl Reply, Rejection> {
//     match User::from_uuid(&pool, user_id).await {
//         Ok(Some(user)) => {
//             // User found: Return user data with 200 OK.
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&user),
//                 StatusCode::OK,
//             ))
//         }
//         Ok(None) => {
//             // User not found: Return 404 Not Found.
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&serde_json::json!({"error": "User not found"})),
//                 StatusCode::NOT_FOUND,
//             ))
//         }
//         Err(err) => {
//             // Database error: Log it and return 500 Internal Server Error.
//             eprintln!("Database error: {}", err);
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&serde_json::json!({"error": "Internal server error"})),
//                 StatusCode::INTERNAL_SERVER_ERROR,
//             ))
//         }
//     }
// }
