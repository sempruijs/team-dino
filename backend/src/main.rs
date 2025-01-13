use crate::controller::authentication::authentication_routes;
use crate::service::authentication::*;
use crate::service::place::*;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
#[allow(warnings)]
use utoipa_swagger_ui::SwaggerUi;
#[macro_use]
use rocket::get;
use rocket::http::Status;
use rocket::routes;
extern crate rocket;
use crate::controller::place::*;
use crate::controller::user::*;
use crate::docs::ApiDoc;
use crate::repository::place::PlaceRepositoryImpl;
use crate::repository::user::UserRepositoryImpl;
use crate::service::user::UserService;
use crate::service::user::UserServiceImpl;
use rocket::Build;
use rocket::State;
use sqlx::PgPool;

pub mod controller;
pub mod docs;
pub mod domain;
pub mod parser;
pub mod repository;
pub mod service;
pub mod traits;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Read the database path from enviousment variables.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("DINO_SECRET_KEY").expect("DINO_SECRET_KEY must be set");

    let pool = PgPool::connect_lazy(&database_url).expect("Failed to connect to the database");

    // Build layers
    let user_repository = UserRepositoryImpl::new(pool.clone());
    let user_service: Arc<dyn UserService> =
        Arc::new(UserServiceImpl::new(user_repository.clone()));

    let place_repository = PlaceRepositoryImpl::new(pool.clone());
    let place_service: Arc<dyn PlaceService> = Arc::new(PlaceServiceImpl::new(place_repository));

    let authentication_service: Arc<dyn AuthenticationService> =
        Arc::new(AuthenticationServiceImpl::new(user_repository.clone()));

    let _rocket = rocket::build()
        .manage(user_service)
        .manage(place_service)
        .manage(authentication_service)
        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/users", user_routes())
        .mount("/login", authentication_routes())
        .mount("/places", place_routes())
        .launch()
        .await?;

    Ok(())
}
