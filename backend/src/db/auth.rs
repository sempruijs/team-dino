use crate::hash::*;
use sqlx::PgPool;

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    // Fetch the hashed password for the given email
    let hashed_password: Option<String> =
        sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

    match hashed_password {
        Some(hashed_password) => Ok(verify_password(password, &hashed_password)),
        None => Ok(false),
    }
}
