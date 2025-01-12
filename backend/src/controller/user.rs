use crate::parser::*;
use crate::repository::user::User;
use crate::service::user::UserService;
use chrono::NaiveDate;
use rocket::futures::FutureExt;
use rocket::post;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub date_of_birth: String,
    pub password: String,
}

// Return type should later be CreateUserRepsonse
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Creates a user. The email should be unique",
    operation_id = "createUser",
    tag = "Users"
)]
#[post("/users", data = "<payload>")]
pub async fn create_user(
    payload: Json<CreateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
) -> Json<bool> {
    // Convert `CreateUserRequest` to `User`
    let user = User {
        user_id: Uuid::new_v4(), // Generate a new UUID for the user
        name: payload.name.clone(),
        date_of_birth: iso8601_str_to_date(&payload.date_of_birth).unwrap(), // Ensure this field has the correct type
        email: payload.email.clone(),
        password: payload.password.clone(),
    };

    // Call the `create` method and await its result
    match user_service.create(user).await {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user]
}
