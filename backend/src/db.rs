use crate::User;
use crate::Ticket;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, u: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (user_id, name, date_of_birth, email, license_plate) VALUES ($1, $2, $3, $4, $5)",
        u.user_id,
        u.name,
        u.date_of_birth,
        u.email,
        serde_json::to_value(&u.license_plate).expect("problem formatting licenseplate vector."),
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_ticket(pool: &PgPool, t: Ticket) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tickets (ticket_id, user_id, start_date, end_date, house_number) VALUES ($1, $2, $3, $4, $5)",
        t.ticket_id,
        t.user_id,
        t.start_date,
        t.end_date,
        t.house_number as i32,
    )
    .execute(pool)
    .await?;

    Ok(())
}