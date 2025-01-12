use crate::UserService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
// use utoipa::ToSchema;
use utoipa::ToSchema;
// use utoipa_rocket::path;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub date_of_birth: String,
    pub password: String,
}

// #[utoipa::path(
//     post,
//     path = "/users",
//     request_body = CreateUser,
//     responses(
//         (status = 201, description = "User created successfully"),
//         (status = 400, description = "Bad request"),
//         (status = 500, description = "Internal server error")
//     ),
//     operation_id = "create_user",
//     tag = "Users"
// )]

pub struct UserController {
    user_service: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
}

// pub async fn get_user_handler(pool: PgPool, user_id: Uuid) -> Result<impl Reply, Rejection> {
//     match User::from_uuid(&pool, user_id).await {
//         Ok(Some(user)) => {
//             // User found: Return user data with 200 OK.
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&user),
//                 StatusCode::OK,
//             ))
//         }
//         Ok(None) => {
//             // User not found: Return 404 Not Found.
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&serde_json::json!({"error": "User not found"})),
//                 StatusCode::NOT_FOUND,
//             ))
//         }
//         Err(err) => {
//             // Database error: Log it and return 500 Internal Server Error.
//             eprintln!("Database error: {}", err);
//             Ok(warp::reply::with_status(
//                 warp::reply::json(&serde_json::json!({"error": "Internal server error"})),
//                 StatusCode::INTERNAL_SERVER_ERROR,
//             ))
//         }
//     }
// }
