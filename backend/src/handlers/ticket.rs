use crate::db::ticket::*;
use crate::logging::current_time_iso8601;
use crate::types::ticket::*;
use sqlx::PgPool;
use warp::http::StatusCode;

pub async fn create_ticket_handler(
    ticket: Ticket,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601(); // Assuming this function returns the current time as ISO8601
    println!(
        "New ticket created for user_id: {}  ({})",
        ticket.user_id, now
    );

    match create_ticket(&pool, ticket).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => {
            panic!("Error while creating ticket");
        }
    }
}
