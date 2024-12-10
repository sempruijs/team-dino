use crate::types::place::*;
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
