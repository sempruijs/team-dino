use crate::domain::Place;
use crate::repository::place::PlaceRepository;
use rocket::async_trait;

#[async_trait]
pub trait PlaceService: Send + Sync {
    // async fn create(&self, user: User) -> Result<(), sqlx::Error>;
    // async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn get_places(&self) -> Result<Vec<Place>, sqlx::Error>;
}

pub struct PlaceServiceImpl<R: PlaceRepository> {
    place_repository: R,
}

impl<R: PlaceRepository> PlaceServiceImpl<R> {
    pub fn new(place_repository: R) -> Self {
        Self { place_repository }
    }
}

#[async_trait]
impl<R: PlaceRepository> PlaceService for PlaceServiceImpl<R> {
    async fn get_places(&self) -> Result<Vec<Place>, sqlx::Error> {
        self.place_repository.get_places().await
    }
}
