use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(create_user_handler), components(schemas(CreateUser)))]
pub struct ApiDoc;
