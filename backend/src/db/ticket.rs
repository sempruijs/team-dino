pub async fn create_ticket(pool: &PgPool, ticket: Ticket) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tickets (ticket_id, user_id, start_date, end_date, house_number)
         VALUES ($1, $2, $3, $4, $5)",
        ticket.ticket_id,
        ticket.user_id,
        ticket.start_date,
        ticket.end_date,
        ticket.house_number
    )
    .execute(pool)
    .await?;
    Ok(())
}
