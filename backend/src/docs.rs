use crate::controller::authentication::*;
use crate::controller::place::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_user, get_places, login, ), // Add authenticate_user_handler here
    components(
        schemas(CreateUserRequest) // Add the schemas
    )
)]
pub struct ApiDoc;
