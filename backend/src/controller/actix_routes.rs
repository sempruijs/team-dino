use crate::controller::auth::*;
use crate::controller::user::*;
use actix_web::Scope;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn user() -> Scope {
    web::scope("/users") // Prefix all routes with `/users`
        .route("", web::post().to(create_user_handler)) // POST /users
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn serve_actix_routes(pool: PgPool, secret_key: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(hello)
            .service(authenticate)
            .service(user())
            .service(echo)
            .service(SwaggerUi::new("/docs/{_:.*}").urls(vec![(
                Url::new("docs", "/docs/openapi1.json"),
                crate::docs::ApiDoc::openapi(),
            )]))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
