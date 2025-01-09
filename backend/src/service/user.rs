use crate::controller::user::*;
use crate::repository::user::*;
use crate::traits::*;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, request: CreateUser) -> Result<(), sqlx::Error> {
    let user_id = Uuid::new_v4();
    let user = User {
        name: request.name,
        date_of_birth: request.date_of_birth,
        email: request.email,
        password: request.password,
        user_id,
    };
    user.create(&pool).await
}
