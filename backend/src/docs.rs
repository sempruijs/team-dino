use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_user), // Add authenticate_user_handler here
    components(
        schemas(CreateUserRequest) // Add the schemas
    )
)]
pub struct ApiDoc;
