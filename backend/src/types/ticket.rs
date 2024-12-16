use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub place_id: Uuid,
}

impl Ticket {
    pub fn valid(&self) -> bool {
        let today = Local::now().date_naive(); // Get the current date
        self.start_date <= today && today <= self.end_date
    }
}
