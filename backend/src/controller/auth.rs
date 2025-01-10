use crate::repository::auth::*;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct AuthenticateUserResponse {
    pub valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateUserRequest {
    pub email: String,
    pub password: String,
}

#[post("/authenticate")]
pub async fn authenticate_user_handler(
    request: web::Json<AuthenticateUserRequest>, // Deserialize the JSON request body
    pool: web::Data<PgPool>,                     // Inject the database pool
    secret_key: web::Data<String>,               // Inject the secret key (for JWT signing)
) -> impl Responder {
    // Use your existing function to authenticate the user
    match authenticate_user(
        &pool,
        &request.email,
        &request.password,
        secret_key.to_string(),
    )
    .await
    {
        Ok(valid) => HttpResponse::Ok().json(AuthenticateUserResponse {
            valid: valid.is_some(),
        }),
        Err(e) => {
            // Log the error and return an internal server error response
            eprintln!("Error checking user credentials: {}", e);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
    }
}
