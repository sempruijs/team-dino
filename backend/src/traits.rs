use sqlx::PgPool;
use uuid::Uuid;

/// Takes a Uuid and returns an Result of Option<Self>
/// Usefull for http get requests.
pub trait FromUuid {
    fn from_uuid(
        pool: &PgPool,
        uuid: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Self: Sized;
}

pub trait Create {
    fn create(
        self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send
    where
        Self: Sized;
}
