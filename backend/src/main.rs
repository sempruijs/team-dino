#[allow(warnings)]
use std::sync::Arc;
#[macro_use]
use rocket::get;
use rocket::http::Status;
use rocket::routes;
extern crate rocket;
use crate::controller::user::*;
use crate::repository::user::UserRepositoryImpl;
use crate::service::user::UserServiceImpl;
use rocket::Build;
use rocket::State;
use sqlx::PgPool;

pub mod controller;
pub mod repository;
pub mod service;
pub mod traits;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, saas!"
}

// #[post("/users", data = "<user>")]
// pub async fn create_user_route(
//     user: Json<User>,
//     controller: &State<UserController>, // Access controller via state
// ) -> Result<Status, (Status, String)> {
//     match controller.create(user.into_inner()).await {
//         Ok(_) => Ok(Status::Created),
//         Err(err) => Err((Status::InternalServerError, err)),
//     }
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = PgPool::connect_lazy("postgres://sem@host.orb.internal:5432/sem")
        .expect("Failed to connect to the database");

    // Build layers
    let repository = UserRepositoryImpl::new(pool);
    let user_service = UserServiceImpl::new(repository);

    let _rocket = rocket::build()
        .manage(user_service.clone())
        .mount("/", user_routes(Arc::new(user_service)))
        .launch()
        .await?;

    Ok(())
}
