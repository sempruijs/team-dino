use std::sync::Arc;
use utoipa::OpenApi;
#[allow(warnings)]
use utoipa_swagger_ui::SwaggerUi;
#[macro_use]
use rocket::get;
use rocket::http::Status;
use rocket::routes;
extern crate rocket;
use crate::controller::user::*;
use crate::docs::ApiDoc;
use crate::repository::user::UserRepositoryImpl;
use crate::service::user::UserService;
use crate::service::user::UserServiceImpl;
use rocket::Build;
use rocket::State;
use sqlx::PgPool;

pub mod controller;
pub mod docs;
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = PgPool::connect_lazy("postgres://sem@host.orb.internal:5432/sem")
        .expect("Failed to connect to the database");

    // Build layers
    let repository = UserRepositoryImpl::new(pool);
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(repository));

    let _rocket = rocket::build()
        .manage(user_service)
        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", user_routes())
        .launch()
        .await?;

    Ok(())
}
