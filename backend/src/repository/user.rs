use crate::repository::ticket::get_tickets;
use crate::service::hash::*;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub password: String,
}

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
    let tickets = get_tickets(pool, user_id).await?;
    // Check if any ticket is valid
    Ok(tickets.iter().any(|ticket| ticket.valid()))
}

pub async fn get_user_uuid_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    // Query the database for the user's UUID based on their email
    let user_uuid = sqlx::query_scalar!("SELECT user_id FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await?;

    Ok(user_uuid)
}
