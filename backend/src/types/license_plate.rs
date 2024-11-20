use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LicensePlate {
    pub plate_id: i32,
    pub user_id: Uuid,
    pub license_plate: String,
}

#[derive(Serialize)]
pub struct LicensePlateResponse {
    pub exists: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLicensePlateRequest {
    pub user_id: Uuid,
    pub license_plate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicensePlateRequest {
    pub license_plate: String,
}
