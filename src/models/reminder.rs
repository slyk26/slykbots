use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};

#[derive(Debug, FromRow)]
pub struct Reminder {
    pub id: i32,
    pub channel_id: String,
    pub user_id: String,
    pub message: Option<String>,
    pub remind_at: DateTime<Utc>,
    pub state: State
}

#[derive(Debug, Type, PartialOrd, PartialEq, Clone)]
#[sqlx(type_name= "state")]
pub enum State {
    ACTIVE,
    ABORTED,
    DONE,
    INVALID
}