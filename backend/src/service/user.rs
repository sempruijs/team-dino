use crate::controller::user::*;
use crate::domain::User;
use crate::repository::user::*;
use crate::traits::*;
use rocket::async_trait;

#[async_trait]
pub trait UserService: Send + Sync {
    // should be a service error.
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: T,
}

// pub async fn create_user(pool: &PgPool, request: CreateUser) -> Result<(), sqlx::Error> {
//     let user_id = Uuid::new_v4();
//     // date parsing should happen incontroller.
//     let date: Vec<i32> = request
//         .date_of_birth
//         .split("-")
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();
//     let date_of_birth = NaiveDate::from_ymd_opt(
//         *date.get(0).unwrap(),
//         (*date.get(1).unwrap()).try_into().unwrap(),
//         (*date.get(2).unwrap()).try_into().unwrap(),
//     )
//     .unwrap();
//     let user = User {
//         name: request.name,
//         date_of_birth,
//         email: request.email,
//         password: request.password,
//         user_id,
//     };
//     user.create(&pool).await
// }

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<R: UserRepository> UserService for UserServiceImpl<R> {
    // should be a service error.
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        self.user_repository.create(user).await
    }
}
