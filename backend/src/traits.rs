use uuid::Uuid;

pub trait FromUuid {
    fn from_uuid(uuid: Uuid) -> Option<Self>
    where
        Self: Sized;
}
