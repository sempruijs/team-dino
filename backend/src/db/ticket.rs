use crate::types::ticket::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

// creates a ticket only if the place is not taken by someone else.
pub async fn create_ticket(pool: &PgPool, ticket: Ticket) -> Result<(), sqlx::Error> {
    // Check if the place is available for the given dates
    let overlapping_tickets = sqlx::query!(
        r#"
        SELECT 1 AS exists
        FROM tickets
        WHERE place_id = $1
          AND (($2::DATE, $3::DATE) OVERLAPS (start_date, end_date))
        "#,
        ticket.place_id,
        ticket.start_date,
        ticket.end_date
    )
    .fetch_optional(pool)
    .await?;

    // If there are overlapping tickets, the place is not available
    if overlapping_tickets.is_some() {
        return Err(sqlx::Error::RowNotFound); // Use an appropriate error type for your application
    }

    // If the place is available, insert the ticket
    sqlx::query!(
        "INSERT INTO tickets (ticket_id, user_id, start_date, end_date, place_id)
         VALUES ($1, $2, $3, $4, $5)",
        ticket.ticket_id,
        ticket.user_id,
        ticket.start_date,
        ticket.end_date,
        ticket.place_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_tickets(pool: &PgPool, user_id: Uuid) -> Result<Vec<Ticket>, sqlx::Error> {
    // Query all tickets associated with the user
    let tickets = sqlx::query_as!(
        Ticket,
        r#"
        SELECT ticket_id, user_id, start_date, end_date, place_id
        FROM tickets
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Check if any ticket is valid
    Ok(tickets)
}
