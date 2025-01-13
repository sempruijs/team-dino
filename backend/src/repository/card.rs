use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait CardRepository: Send + Sync {
    async fn get_user_id(&self, card_id: i32) -> Result<Option<Uuid>, sqlx::Error>;
}

pub struct CardRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl CardRepository for CardRepositoryImpl {
    async fn get_user_id(&self, card_id: i32) -> Result<Option<Uuid>, sqlx::Error> {
        // Query the database for the UUID associated with the given license plate
        let query_result = sqlx::query_scalar!(
            r#"
            SELECT user_id
            FROM cards
            WHERE id = $1
            "#,
            card_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(query_result)
    }
}

// pub async fn check_card(pool: &PgPool, card_id: &str) -> Result<bool, sqlx::Error> {
//     let exists = sqlx::query_scalar!(
//         r#"
//         SELECT EXISTS(
//             SELECT 1
//             FROM cards
//             WHERE card_id = $1
//         )
//         "#,
//         card_id
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(exists.unwrap())
// }

// pub async fn create_card(pool: &PgPool, user_id: Uuid, card_id: String) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
//         INSERT INTO cards (user_id, card_id)
//         VALUES ($1, $2)
//         "#,
//         user_id,
//         card_id
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }
