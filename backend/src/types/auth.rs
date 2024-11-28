use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthenticateUserResponse {
    pub valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateUserRequest {
    pub email: String,
    pub password: String,
}
