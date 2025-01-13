use crate::repository::license_plate::*;
use crate::repository::user::UserRepository;
use chrono::NaiveDate;
use rocket::async_trait;

use crate::repository::license_plate::{self, LicensePlateRepository};

#[async_trait]
pub trait LicensePlateService: Send + Sync {
    async fn valid_on_date(
        &self,
        license_plate: String,
        date: NaiveDate,
    ) -> Result<bool, sqlx::Error>;
}

impl<U: UserRepository, L: LicensePlateRepository> LicensePlateServiceImpl<U, L> {
    pub fn new(user_repository: U, license_plate_repository: L) -> Self {
        Self {
            user_repository,
            license_plate_repository,
        }
    }
}

pub struct LicensePlateServiceImpl<U: UserRepository, L: LicensePlateRepository> {
    license_plate_repository: L,
    user_repository: U,
}

#[async_trait]
impl<U: UserRepository, L: LicensePlateRepository> LicensePlateService
    for LicensePlateServiceImpl<U, L>
{
    async fn valid_on_date(
        &self,
        license_plate: String,
        date: NaiveDate,
    ) -> Result<bool, sqlx::Error> {
        let user_id = self
            .license_plate_repository
            .get_user_id(license_plate)
            .await?;
        match user_id {
            None => return Ok(false),
            Some(user_id) => Ok(self.user_repository.allowed_on_date(user_id, date).await?),
        }
    }
}
