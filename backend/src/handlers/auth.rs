use crate::db::auth::*;
use sqlx::PgPool;

pub async fn authenticate_user_handler(
    email: String,
    password: String,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match authenticate_user(&pool, &email, &password).await {
        Ok(valid) => Ok(warp::reply::json(&valid)),
        Err(e) => panic!("Error checking license plate: {}", e),
    }
}
