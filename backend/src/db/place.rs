use crate::types::place::*;
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

// pub async fn get_tickets(pool: &PgPool, user_id: Uuid) -> Result<Vec<Ticket>, sqlx::Error> {
//     // Query all tickets associated with the user
//     let tickets = sqlx::query_as!(
//         Ticket,
//         r#"
//         SELECT ticket_id, user_id, start_date, end_date, house_number
//         FROM tickets
//         WHERE user_id = $1
//         "#,
//         user_id
//     )
//     .fetch_all(pool)
//     .await?;

//     // Check if any ticket is valid
//     Ok(tickets)
// }
