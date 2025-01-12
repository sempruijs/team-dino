use crate::service::user::UserService;
use rocket::post;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

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
    operation_id = "createUser",
    tag = "Users"
)]
#[post("/users", data = "<payload>")]
pub async fn create_user(
    payload: Json<CreateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
) -> Json<bool> {
    Json(true)
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user]
}
