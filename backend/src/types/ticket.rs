#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub house_number: i32,
}
