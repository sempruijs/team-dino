use crate::LicensePlate;
use crate::Ticket;
use crate::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (user_id, name, date_of_birth, email)
         VALUES ($1, $2, $3, $4)",
        user.user_id,
        user.name,
        user.date_of_birth,
        user.email
    )
    .execute(pool)
    .await?;
    Ok(())
}

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

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT user_id, name, date_of_birth, email
         FROM users
         WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn check_license_plate(pool: &PgPool, plate: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM license_plates 
            WHERE license_plate = $1
        )
        "#,
        plate
    )
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap())
}

pub async fn check_card(pool: &PgPool, card_id: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM cards 
            WHERE card_id = $1
        )
        "#,
        card_id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap())
}

pub async fn create_license_plate(
    pool: &PgPool,
    user_id: Uuid,
    license_plate: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO license_plates (user_id, license_plate)
        VALUES ($1, $2)
        "#,
        user_id,
        license_plate
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_card(pool: &PgPool, user_id: Uuid, card_id: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO cards (user_id, card_id)
        VALUES ($1, $2)
        "#,
        user_id,
        card_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
