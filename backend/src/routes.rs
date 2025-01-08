use crate::db::auth::*;
use crate::handlers::auth::*;
use crate::handlers::card::*;
use crate::handlers::license_plate::*;
use crate::handlers::place::*;
use crate::handlers::ticket::*;
use crate::handlers::user::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::Filter;
use warp::Rejection;

// warp filter that requires valid jwt
pub fn with_jwt_auth(
    secret_key: String,
) -> impl Filter<Extract = (Uuid,), Error = Rejection> + Clone {
    warp::header::<String>("authorization").and_then(move |token: String| {
        let secret_key = secret_key.clone(); // Clone the secret_key inside the closure
        async move {
            let token = token.trim_start_matches("Bearer ");
            match verify_jwt(token, secret_key) {
                Ok(claims) => {
                    let user_id =
                        Uuid::parse_str(&claims.uuid).expect("error while parsing string  to uuid"); // Ensure claims contain `user_id`
                    Ok(user_id)
                } // Proceed if JWT is valid
                Err(_) => Err(warp::reject::custom(Unauthorized)), // Reject if JWT is invalid
            }
        }
    })
}

// Custom rejection type for unauthorized access
#[derive(Debug)]
struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

// Function to handle the rejection
// async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
//     if let Some(Unauthorized) = err.find::<Unauthorized>() {
//         Ok(warp::reply::with_status(
//             "Unauthorized",
//             StatusCode::UNAUTHORIZED,
//         ))
//     } else {
//         // Handle other rejections
//         Ok(warp::reply::with_status(
//             "Internal Server Error",
//             StatusCode::INTERNAL_SERVER_ERROR,
//         ))
//     }
// }

pub async fn serve_routes(pool: PgPool, secret_key: String) {
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
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(create_ticket_handler);

    // check_license_plate endpoint
    let check_license_plate = warp::get()
        .and(warp::path("check_license_plate"))
        .and(warp::path::param()) // Path parameter (license plate)
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(check_license_plate_handler);

    // check if card is valid endpoint
    let check_card = warp::get()
        .and(warp::path("check_card"))
        .and(warp::path::param())
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(check_card_handler);

    // add a license plate to a user endpoint
    let create_license_plate = warp::post()
        .and(warp::path("license_plates")) // Exposes the route at /license_plates
        .and(warp::body::json()) // Expects a JSON body
        .and(pool_filter.clone()) // Provides the database connection pool
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(create_license_plate_handler); // Calls the handler

    // For associateding an nfc card to a user endpoint.
    let create_card = warp::post()
        .and(warp::path("cards"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(create_card_handler);

    // For creating a new place
    let create_place = warp::post()
        .and(warp::path("create_place"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(create_place_handler);

    // For recieving all places
    let get_places = warp::get()
        .and(warp::path("places"))
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(get_places_handler);

    // for recieving places with a date filter
    let get_available_places = warp::get()
        .and(warp::path("places"))
        .and(warp::path("available"))
        .and(warp::query::<AvailablePlacesQuery>()) // Deserialize query parameters
        .and(pool_filter.clone())
        .and_then(get_available_places_handler);

    // For deleting a place
    let delete_place = warp::delete()
        .and(warp::path("places"))
        .and(warp::path::param::<Uuid>())
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(delete_place_handler);

    // recieving user by uuid
    let get_user = warp::get()
        .and(warp::path("users"))
        .and(pool_filter.clone())
        .and(with_jwt_auth(secret_key.clone()))
        .and_then(get_user_handler);

    // check if user credentials are valid endpoint
    let authenticate_user = warp::post()
        .and(warp::path("authenticate"))
        .and(warp::body::json()) // Parse JSON body containing email and password
        .and(pool_filter.clone()) // Database connection pool filter
        .and(warp::any().map(move || secret_key.clone()))
        .and_then(authenticate_user_handler);

    // Combine all the routes
    let routes = create_user
        .or(create_ticket)
        .or(check_license_plate)
        .or(check_card)
        .or(create_license_plate)
        .or(create_card)
        .or(create_place)
        .or(get_places)
        .or(get_available_places)
        .or(delete_place)
        .or(get_user)
        .or(authenticate_user)
        .with(cors);

    println!("Running on port 3030...");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
