use crate::db::user::*;
use crate::logging::*;
use crate::types::user::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;

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

pub async fn get_user_handler(user_id: Uuid, pool: PgPool) -> Result<impl Reply, Rejection> {
    match get_user_by_id(&pool, user_id).await {
        Ok(Some(user)) => {
            // User found: Return user data with 200 OK.
            Ok(warp::reply::with_status(
                warp::reply::json(&user),
                StatusCode::OK,
            ))
        }
        Ok(None) => {
            // User not found: Return 404 Not Found.
            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "User not found"})),
                StatusCode::NOT_FOUND,
            ))
        }
        Err(err) => {
            // Database error: Log it and return 500 Internal Server Error.
            eprintln!("Database error: {}", err);
            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "Internal server error"})),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
