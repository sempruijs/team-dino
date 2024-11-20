use crate::db::user::*;
use crate::logging::*;
use crate::types::user::*;
use sqlx::PgPool;
use warp::http::StatusCode;

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
