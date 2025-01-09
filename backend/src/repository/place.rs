use crate::types::place::*;
use chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn create_place(pool: &PgPool, place: Place) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO places (place_id, house_number)
         VALUES ($1, $2)",
        place.place_id,
        place.house_number
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_places(pool: &PgPool) -> Result<Vec<Place>, sqlx::Error> {
    let places = sqlx::query_as!(
        Place,
        r#"
        SELECT place_id, house_number
        FROM places
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(places)
}

pub async fn delete_place(pool: &PgPool, place_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM places
        WHERE place_id = $1
        "#,
        place_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_available_places(
    pool: &PgPool,
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
    .fetch_all(pool)
    .await?;

    Ok(places)
}
