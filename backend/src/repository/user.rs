// use crate::repository::ticket::get_tickets;
use crate::domain::User;
use crate::service::hash::*;
use crate::traits::FromUuid;
use crate::traits::*;
use rocket::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, name, date_of_birth, email, password)
             VALUES ($1, $2, $3, $4, $5)",
            user.user_id,
            user.name,
            user.date_of_birth,
            user.email,
            hash_password(&user.password)
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, date_of_birth, email, password
             FROM users
             WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

// impl User {
//     // Could be better, valid should recieve self and a pool,
//     // The service layer should merge it into this function.
//     pub async fn valid(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
//         let tickets = get_tickets(pool, user_id).await?;
//         // Check if any ticket is valid
//         Ok(tickets.iter().any(|ticket| ticket.valid()))
//     }
// }

// impl Create for User {
//     async fn create(self, pool: &PgPool) -> Result<(), sqlx::Error> {
//         sqlx::query!(
//             "INSERT INTO users (user_id, name, date_of_birth, email, password)
//              VALUES ($1, $2, $3, $4, $5)",
//             self.user_id,
//             self.name,
//             self.date_of_birth,
//             self.email,
//             hash_password(&self.password)
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }
// }

impl FromUuid for User {
    async fn from_uuid(pool: &PgPool, user_id: Uuid) -> Result<Option<Self>, sqlx::Error>
    where
        Self: Sized,
    {
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
}

// pub async fn get_user_uuid_by_email(
//     pool: &PgPool,
//     email: &str,
// ) -> Result<Option<Uuid>, sqlx::Error> {
//     // Query the database for the user's UUID based on their email
//     let user_uuid = sqlx::query_scalar!("SELECT user_id FROM users WHERE email = $1", email)
//         .fetch_optional(pool)
//         .await?;

//     Ok(user_uuid)
// }
