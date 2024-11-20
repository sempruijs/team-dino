pub async fn check_license_plate_handler(
    plate: String, // Assuming you have a struct LicensePlateRequest for deserialization
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match check_license_plate(&pool, &plate).await {
        Ok(exists) => {
            if exists {
                Ok(warp::reply::json(&true)) // If plate exists, return true
            } else {
                Ok(warp::reply::json(&false)) // If plate does not exist, return false
            }
        }
        Err(e) => {
            panic!("Error checking license plate: {}", e);
        }
    }
}

pub async fn create_license_plate_handler(
    request: CreateLicensePlateRequest,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    match create_license_plate(&pool, request.user_id, request.license_plate).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "License plate created successfully"})),
            StatusCode::CREATED,
        )),
        Err(err) => {
            eprintln!("Database error: {}", err);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "Failed to create license plate"})),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
