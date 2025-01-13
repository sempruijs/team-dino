use crate::repository::user::*;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait LicensePlateRepository: Send + Sync {
    async fn get_user_id(&self, license_plate: String) -> Result<Option<Uuid>, sqlx::Error>;
}

pub struct LicensePlateRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl LicensePlateRepository for LicensePlateRepositoryImpl {
    async fn get_user_id(&self, license_plate: String) -> Result<Option<Uuid>, sqlx::Error> {
        // Query the database for the UUID associated with the given license plate
        let query_result = sqlx::query_scalar!(
            r#"
            SELECT user_id
            FROM license_plates
            WHERE license_plate = $1
            "#,
            license_plate
        )
        .fetch_optional(&self.pool) // Use `fetch_optional` to return an Option
        .await?;

        Ok(query_result) // Return the result directly
    }
}

// pub async fn license_plate_is_valid(pool: &PgPool, plate: &str) -> Result<bool, sqlx::Error> {
//     let uuid = get_uuid_from_license_plate(pool, plate).await?;
//     if let Some(user_id) = uuid {
//         return Ok(User::valid(pool, user_id).await?);
//     }
//     Ok(false)
// }

// pub async fn get_uuid_from_license_plate(
//     pool: &PgPool,
//     plate: &str,
// ) -> Result<Option<Uuid>, sqlx::Error> {
//     let result = sqlx::query_scalar!(
//         "SELECT user_id
//          FROM license_plates
//          WHERE license_plate = $1",
//         plate
//     )
//     .fetch_optional(pool)
//     .await?;

//     Ok(result)
// }

// pub async fn create_license_plate(
//     pool: &PgPool,
//     user_id: Uuid,
//     license_plate: String,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
//         INSERT INTO license_plates (user_id, license_plate)
//         VALUES ($1, $2)
//         "#,
//         user_id,
//         license_plate
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }
