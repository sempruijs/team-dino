use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
    // Generate a hashed password
    hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // Verify the password against the hashed password
    verify(password, hashed_password).unwrap_or(false)
}

