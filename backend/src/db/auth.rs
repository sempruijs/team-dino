use sqlx::PgPool;
use argon2::{self, Config, verify_encoded};

pub async fn authenticate_user(pool: &PgPool, email: &str, password: &str) -> Result<bool, sqlx::Error> {
    // Fetch the hashed password for the given email
    if let Some(hashed_password) = sqlx::query_scalar!(
        "SELECT password FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?
    {
        // Verify the provided password against the hashed password
        match verify_encoded(&hashed_password, password.as_bytes()) {
            Ok(valid) => Ok(valid), // true if passwords match
            Err(_) => Ok(false),   // false if verification fails
        }
    } else {
        // Email does not exist
        Ok(false)
    }
} W
