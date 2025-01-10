use crate::controller::auth::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_user_handler, authenticate), // Add authenticate_user_handler here
    components(
        schemas(CreateUser, AuthenticateUserRequest, AuthenticateUserResponse) // Add the schemas
    )
)]
pub struct ApiDoc;
