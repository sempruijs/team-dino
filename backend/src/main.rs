#[macro_use]
use rocket::http::Status;
extern crate rocket;
use crate::controller::user::*;
use crate::repository::user::UserRepository;
use crate::service::user::UserService;
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

#[post("/users", data = "<user>")]
pub async fn create_user_route(
    user: Json<User>,
    controller: &State<UserController>, // Access controller via state
) -> Result<Status, (Status, String)> {
    match controller.create(user.into_inner()).await {
        Ok(_) => Ok(Status::Created),
        Err(err) => Err((Status::InternalServerError, err)),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = PgPool::connect_lazy("postgres://sem@host.orb.internal:5432/sem")
        .expect("Failed to connect to the database");

    // Build layers
    let repository = UserRepository::new(pool);
    let service = UserService::new(repository);
    let controller = UserController::new(service);

    let _rocket = rocket::build()
        .manage(controller) // Inject UserController as managed state
        .mount("/", routes![create_user_route])
        .launch()
        .await?;

    Ok(())
}
