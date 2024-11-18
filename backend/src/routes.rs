use sqlx::PgPool;
use warp::Filter;

use crate::handlers::*;

pub async fn serve_routes(pool: PgPool) {
    // Clone the pool to share it across routes
    let pool_filter = warp::any().map(move || pool.clone());

    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin
        .allow_header("content-type") // Allow specific headers
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]); // Allow specific methods

    // POST /users - Create a new user
    let create_user = warp::post()
        .and(warp::path("users"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(create_user_handler);

    let create_ticket = warp::post()
        .and(warp::path("create_ticket"))
        .and(warp::body::json()) // Accept JSON body for ticket
        .and(pool_filter.clone())
        .and_then(create_ticket_handler);

    let check_license_plate = warp::get()
        .and(warp::path("check_license_plate"))
        .and(warp::path::param()) // Path parameter (license plate)
        .and(pool_filter.clone())
        .and_then(check_license_plate_handler);

    // Combine all the routes
    let routes = create_user
        .or(create_ticket)
        .or(check_license_plate)
        .with(cors);

    println!("Starting server");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
