use crate::handlers::auth::*;
use crate::handlers::card::*;
use crate::handlers::license_plate::*;
use crate::handlers::place::*;
use crate::handlers::ticket::*;
use crate::handlers::user::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::Filter;

pub async fn serve_routes(pool: PgPool) {
    // Clone the pool to share it across routes
    let pool_filter = warp::any().map(move || pool.clone());

    // All requests from any origin.
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

    // Create ticket endpoint
    let create_ticket = warp::post()
        .and(warp::path("create_ticket"))
        .and(warp::body::json()) // Accept JSON body for ticket
        .and(pool_filter.clone())
        .and_then(create_ticket_handler);

    // check_license_plate endpoint
    let check_license_plate = warp::get()
        .and(warp::path("check_license_plate"))
        .and(warp::path::param()) // Path parameter (license plate)
        .and(pool_filter.clone())
        .and_then(check_license_plate_handler);

    // check if card is valid endpoint
    let check_card = warp::get()
        .and(warp::path("check_card"))
        .and(warp::path::param())
        .and(pool_filter.clone())
        .and_then(check_card_handler);

    // add a license plate to a user endpoint
    let create_license_plate = warp::post()
        .and(warp::path("license_plates")) // Exposes the route at /license_plates
        .and(warp::body::json()) // Expects a JSON body
        .and(pool_filter.clone()) // Provides the database connection pool
        .and_then(create_license_plate_handler); // Calls the handler

    // check if user credentials are valid endpoint
    let authenticate_user = warp::post()
        .and(warp::path("authenticate"))
        .and(warp::body::json()) // Parse JSON body containing email and password
        .and(pool_filter.clone()) // Database connection pool filter
        .and_then(authenticate_user_handler);

    // For associateding an nfc card to a user endpoint.
    let create_card = warp::post()
        .and(warp::path("cards"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(create_card_handler);

    // For creating a new place
    let create_place = warp::post()
        .and(warp::path("create_place"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(create_place_handler);

    // For recieving all places
    let get_places = warp::get()
        .and(warp::path("places"))
        .and(pool_filter.clone())
        .and_then(get_places_handler);

    // recieving user by uuid
    let get_user = warp::get()
        .and(warp::path("users"))
        .and(warp::path::param::<Uuid>()) // User ID as a path parameter.
        .and(pool_filter.clone())
        .and_then(get_user_handler);

    // Combine all the routes
    let routes = create_user
        .or(create_ticket)
        .or(check_license_plate)
        .or(check_card)
        .or(create_license_plate)
        .or(create_card)
        .or(create_place)
        .or(get_places)
        .or(get_user)
        .or(authenticate_user)
        .with(cors);

    println!("Running on port 3030...");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
