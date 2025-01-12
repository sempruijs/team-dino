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
#[post("/users", data = "<payload>")]
async fn create_user(
    payload: Json<CreateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
) -> Json<bool> {
    Json(true)
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user]
}
