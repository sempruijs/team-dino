use crate::controller::user::*;
use crate::repository::user::*;
use crate::traits::*;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, request: CreateUser) -> Result<(), sqlx::Error> {
    let user_id = Uuid::new_v4();
    let date: Vec<i32> = request
        .date_of_birth
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let date_of_birth = NaiveDate::from_ymd_opt(
        *date.get(0).unwrap(),
        (*date.get(1).unwrap()).try_into().unwrap(),
        (*date.get(2).unwrap()).try_into().unwrap(),
    )
    .unwrap();
    let user = User {
        name: request.name,
        date_of_birth,
        email: request.email,
        password: request.password,
        user_id,
    };
    user.create(&pool).await
}
