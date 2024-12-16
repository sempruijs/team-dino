use crate::db::ticket::*;
use crate::logging::current_time_iso8601;
use crate::types::ticket::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use warp::http::StatusCode;

pub async fn create_ticket_handler(
    request: CreateTicket,
    pool: PgPool,
    user_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601(); // Assuming this function returns the current time as ISO8601
    println!("New ticket created for user_id: {}  ({})", user_id, now);

    match create_ticket(&pool, request, user_id).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            panic!("Error while creating ticket {}", e);
        }
    }
}
