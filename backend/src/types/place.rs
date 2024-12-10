use sqlx::types::Uuid;

pub struct Place {
    pub place_id: Uuid,
    pub house_number: i32,
}
