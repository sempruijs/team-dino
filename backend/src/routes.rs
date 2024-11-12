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
        .and(warp::any().map(move || pool.clone())) // Pass the database pool to the handler
        .and_then(create_ticket_handler)

    // Combine all the routes
    let routes = create_user
        .or(create_ticket)
        .with(cors);

    println!("Starting server");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
