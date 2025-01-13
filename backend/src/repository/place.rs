use crate::domain::Place;
use chrono::NaiveDate;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait PlaceRepository: Send + Sync {
    async fn create(&self, place: Place) -> Result<(), sqlx::Error>;

    async fn get_places(&self) -> Result<Vec<Place>, sqlx::Error>;

    async fn get_available_places(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Place>, sqlx::Error>;

    async fn delete(&self, place_id: Uuid) -> Result<u64, sqlx::Error>;

    async fn is_taken_between(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        place_id: Uuid,
    ) -> Result<bool, sqlx::Error>;
}

pub struct PlaceRepositoryImpl {
    pool: PgPool,
}

impl PlaceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlaceRepository for PlaceRepositoryImpl {
    async fn get_places(&self) -> Result<Vec<Place>, sqlx::Error> {
        let places = sqlx::query_as!(
            Place,
            r#"
                    SELECT place_id, house_number
                    FROM places
                    "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(places)
    }

    async fn create(&self, place: Place) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO places (place_id, house_number)
                     VALUES ($1, $2)",
            place.place_id,
            place.house_number
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_available_places(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Place>, sqlx::Error> {
        // Query to fetch available places
        let places = sqlx::query_as!(
            Place,
            r#"
                SELECT places.place_id, places.house_number
                FROM places
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM tickets
                    WHERE tickets.place_id = places.place_id
                    AND (
                        (tickets.start_date <= $2 AND tickets.end_date >= $1)
                        OR (tickets.start_date <= $1 AND tickets.end_date >= $2)
                        OR (tickets.start_date >= $1 AND tickets.end_date <= $2)
                    )
                )
                "#,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(places)
    }

    async fn delete(&self, place_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
        DELETE FROM places
        WHERE place_id = $1
        "#,
            place_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn is_taken_between(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        place_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        // Check if the place is available for the given dates
        let overlapping_tickets = sqlx::query!(
            r#"
        SELECT 1 AS exists
        FROM tickets
        WHERE place_id = $1
          AND (($2::DATE, $3::DATE) OVERLAPS (start_date, end_date))
        "#,
            place_id,
            start_date,
            end_date
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(overlapping_tickets.is_some())
    }
}
