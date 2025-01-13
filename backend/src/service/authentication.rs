// use crate::repository::user::*;
use uuid::Uuid;
// use crate::service::hash::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::repository::user::UserRepository;
// use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize, // Expiration time (as a timestamp)
}

#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn login(
        &self,
        email: String,
        password: String,
        secret: String,
    ) -> Result<Option<String>, sqlx::Error>;
}

pub struct AuthenticationServiceImpl<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> AuthenticationServiceImpl<U> {
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<U: UserRepository> AuthenticationService for AuthenticationServiceImpl<U> {
    async fn login(
        &self,
        email: String,
        password: String,
        secret: String,
    ) -> Result<Option<String>, sqlx::Error> {
        let user = match self.user_repository.from_email(email).await? {
            Some(user) => user,
            None => {
                return Ok(None);
            }
        };
        match user.password == password {
            true => Ok(generate_jwt(user.user_id, secret)?),
            false => Ok(None),
        }
    }
}

// todo, should recieve now as argument.
// IMPURE function.
fn generate_jwt(user_id: Uuid, secret: String) -> Result<Option<String>, sqlx::Error> {
    // calculate experation time.
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid time")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_string(),
        exp: expiration,
    };

    // generate jwt
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("JWT creation failed");

    return Ok(Some(token));
}

// pub async fn authenticate_user(
//     pool: &PgPool,
//     email: &str,
//     password: &str,
//     secret_key: String,
// ) -> Result<Option<String>, sqlx::Error> {
//     // Fetch the hashed password for the given email
//     let hashed_password: Option<String> =
//         sqlx::query_scalar!("SELECT password FROM users WHERE email = $1", email)
//             .fetch_optional(pool)
//             .await?;

//     if let Some(hashed_password) = hashed_password {
//         if verify_password(password, &hashed_password) {
//             // credentials are valid so jwt must be generated.
//             // calculate experation time.
//             let expiration = Utc::now()
//                 .checked_add_signed(Duration::hours(24))
//                 .expect("Invalid time")
//                 .timestamp() as usize;

//             // recieve uuid
//             let uuid = get_user_uuid_by_email(pool, email)
//                 .await?
//                 .expect("failed to unwrap uuid based on email.")
//                 .to_string();

//             let claims = Claims {
//                 uuid: uuid,
//                 exp: expiration,
//             };

//             // generate jwt
//             let token = encode(
//                 &Header::default(),
//                 &claims,
//                 &EncodingKey::from_secret(secret_key.as_bytes()),
//             )
//             .expect("JWT creation failed");

//             return Ok(Some(token));
//         }
//     }

//     Ok(None) // Invalid credentials
// }

// pub fn verify_jwt(token: &str, secret_key: String) -> Result<Claims, jsonwebtoken::errors::Error> {
//     decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(secret_key.as_bytes()),
//         &Validation::default(),
//     )
//     .map(|data| data.claims)
// }
