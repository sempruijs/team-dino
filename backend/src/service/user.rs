use crate::controller::user::*;
use crate::domain::User;
use crate::repository::user::*;
use crate::traits::*;
use rocket::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserService: Send + Sync {
    // should be a service error.
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: T,
}

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

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.from_uuid(user_id).await
    }
}
