use crate::db::user::*;
use crate::hash::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Subject (e.g., user ID)
    exp: usize,  // Expiration time (as a timestamp)
}

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
    secret_key: String,
) -> Result<Option<String>, sqlx::Error> {
    // Fetch the hashed password for the given email
    let hashed_password: Option<String> =
        sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

    if let Some(hashed_password) = hashed_password {
        if verify_password(password, &hashed_password) {
            // credentials are valid so jwt must be generated.
            // calculate experation time.
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("Invalid time")
                .timestamp() as usize;

            let uuid = get_user_uuid_by_email(pool, email)
                .await?
                .expect("failed to unwrap uuid based on email.")
                .to_string();

            let claims = Claims {
                sub: uuid,
                exp: expiration,
            };

            // generate jwt
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret_key.as_bytes()),
            )
            .expect("JWT creation failed");

            return Ok(Some(token));
        }
    }

    Ok(None) // Invalid credentials
}

pub fn verify_jwt(token: &str, secret_key: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
