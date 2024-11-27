use bcrypt::{hash, verify};
use sqlx::PgPool;

pub fn hash_password(password: &str) -> String {
    // Generate a hashed password
    hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // Verify the password against the hashed password
    verify(password, hashed_password).unwrap_or(false)
}

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
