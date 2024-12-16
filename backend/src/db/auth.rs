use crate::hash::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (e.g., user ID)
    exp: usize,  // Expiration time (as a timestamp)
}

const SECRET_KEY: &[u8] = b"your-secret-key";

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
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

            let claims = Claims {
                sub: email.to_string(),
                exp: expiration,
            };

            // generate jwt
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET_KEY),
            )
            .expect("JWT creation failed");

            return Ok(Some(token));
        }
    }

    Ok(None) // Invalid credentials
}

// pub async fn authenticate_user(
//     pool: &PgPool,
//     email: &str,
//     password: &str,
// ) -> Result<bool, sqlx::Error> {
//     // Fetch the hashed password for the given email
//     let hashed_password: Option<String> =
//         sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
//             .fetch_optional(pool)
//             .await?;

//     match hashed_password {
//         Some(hashed_password) => Ok(verify_password(password, &hashed_password)),
//         None => Ok(false),
//     }
// }
