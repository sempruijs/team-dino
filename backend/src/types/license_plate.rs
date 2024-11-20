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
    user_id: Uuid,
    license_plate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicensePlateRequest {
    pub license_plate: String,
}
