use crate::db::ticket::{self, get_tickets};
use crate::hash::*;
use crate::types::ticket::Ticket;
use crate::types::user::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (user_id, name, date_of_birth, email, password)
         VALUES ($1, $2, $3, $4, $5)",
        user.user_id,
        user.name,
        user.date_of_birth,
        user.email,
        hash_password(&user.password)
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT user_id, name, date_of_birth, email, password
         FROM users
         WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn user_is_valid(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    // Query all tickets associated with the user
    let tickets = get_tickets(pool, user_id)?;
    // Check if any ticket is valid
    Ok(tickets.iter().any(|ticket| ticket.valid()))
}
