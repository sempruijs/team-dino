use crate::{db::auth::*, types::auth::AuthenticateUserRequest};
use sqlx::PgPool;

pub async fn authenticate_user_handler(
    request: AuthenticateUserRequest,
    pool: PgPool,
    secret_key: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match authenticate_user(&pool, &request.email, &request.password, secret_key).await {
        Ok(valid) => Ok(warp::reply::json(&valid)),
        Err(e) => panic!("Error checking license plate: {}", e),
    }
}
