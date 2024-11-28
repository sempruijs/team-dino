use crate::hash::*;
use sqlx::PgPool;

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    // Fetch the hashed password for the given email
    if let Some(hashed_password) =
        sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?
    {
        Ok(verify_password(&password, &hashed_password))
    } else {
        // Email does not exist
        Ok(false)
    }
}
