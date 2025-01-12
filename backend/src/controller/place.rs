use crate::domain::Place;
use crate::service::place::PlaceService;
use rocket::futures::FutureExt;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetPlacesRequest;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetPlacesResponse {
    places: Vec<GetPlaceResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetPlaceResponse {
    house_number: i32,
}

// Return type should later be CreateUserRepsonse
#[utoipa::path(
    get,
    path = "/places",
    request_body = GetPlacesRequest,
    responses(
        (status = 201, description = "Places recieved successfully", body = GetPlacesResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve all places",
    operation_id = "GetPlaces",
    tag = "Places"
)]
#[get("/places", data = "<payload>")]
pub async fn get_places(
    payload: Json<GetPlacesRequest>,
    place_service: &State<Arc<dyn PlaceService>>,
) -> Result<Json<GetPlacesResponse>, status::Custom<String>> {
    match place_service.get_places().await {
        Ok(places) => Ok(Json(GetPlacesResponse {
            places: places
                .iter()
                .map(|p| GetPlaceResponse {
                    house_number: p.house_number,
                })
                .collect::<Vec<GetPlaceResponse>>(),
        })),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Internal server error".to_string(),
        )),
    }
}

pub fn place_routes() -> Vec<rocket::Route> {
    routes![get_places]
}
