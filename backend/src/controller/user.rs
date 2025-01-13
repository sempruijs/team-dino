use crate::domain::User;
use crate::parser::*;
use crate::service::user::UserService;
use crate::AuthenticationService;
use chrono::NaiveDate;
use rocket::futures::FutureExt;
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
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
    description = "Creates a user. The email should be unique. Date should be in iso_8601 format, so yyyy-mm-dd.",
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUserRequest;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUserResponse {
    name: String,
    email: String,
}

// Return type should later be CreateUserRepsonse
#[utoipa::path(
    get,
    path = "/users",
    request_body = GetUserRequest,
    responses(
        (status = 201, description = "User recieved successfully", body = GetUserResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve user details.",
    operation_id = "createUser",
    tag = "Users",
    security(
        ("jwt_auth" = [])
    )
)]
#[get("/")]
pub async fn get_user(
    // user_service: &State<Arc<dyn UserService>>,
    user: User,
) -> Result<Json<GetUserResponse>, status::Custom<String>> {
    dbg!(&user);

    Ok(Json(GetUserResponse {
        email: user.email,
        name: user.name,
    }))
    // let user_id = match Uuid::parse_str(&payload.user_id) {
    //     Ok(uuid) => uuid,
    //     Err(_) => {
    //         return Err(status::Custom(
    //             Status::BadRequest,
    //             "Invalid user ID".to_string(),
    //         ))
    //     }
    // };

    // match user_service.from_uuid(user_id).await {
    //     Ok(Some(user)) => Ok(Json(GetUserResponse {
    //         email: user.email,
    //         name: user.name,
    //     })),
    //     Ok(None) => Err(status::Custom(
    //         Status::NotFound,
    //         "User not found".to_string(),
    //     )),
    //     Err(_) => Err(status::Custom(
    //         Status::InternalServerError,
    //         "Internal server error".to_string(),
    //     )),
    // }
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user, get_user]
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let authentication_service = request
            .guard::<&State<Arc<dyn AuthenticationService>>>()
            .await
            .unwrap();

        dbg!("{}", request.headers().get_one("Authorization"));
        match request.headers().get_one("Authorization") {
            None => {
                println!("hello");
                Outcome::Error((Status::Unauthorized, ()))
            }
            Some(autherisation_header) => match autherisation_header.strip_prefix("Bearer ") {
                None => todo!(),
                Some(jwt) => match authentication_service
                    .verify_jwt(jwt, String::from("bla"))
                    .await
                {
                    Ok(Some(user)) => request::Outcome::Success(user.clone()),
                    Ok(None) => {
                        println!("hello2");
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                    Err(_) => {
                        println!("hello3");
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                },
            },
        }
    }
}
