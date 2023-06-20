use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use sqlx::{Error, Pool, Postgres, query, Row};
use sqlx::postgres::PgQueryResult;
use tokio::sync::Mutex;
use crate::models::{Reminder, State};

lazy_static! {
    pub static ref ACTIVE_REMINDERS: Arc<Mutex<HashMap<i32, JoinHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct ReminderService;

impl ReminderService {
    pub async fn insert(db: &Pool<Postgres>, reminder: Reminder) -> Result<i32, Error> {
        debug!("inserting reminder for {}", reminder.remind_at);
        match query("insert into reminders \
        (channel_id, user_id, message, remind_at, state) \
        values($1,$2,$3,$4,$5) returning id")
            .bind(reminder.channel_id)
            .bind(reminder.user_id)
            .bind(reminder.message)
            .bind(reminder.remind_at)
            .bind(State::ACTIVE)
            .fetch_one(db).await {
            Ok(result) => Ok(result.get::<i32, _>(0)),
            Err(e) => {Err(e)}
        }
    }

    pub async fn update_state(db: &Pool<Postgres>, id: i32, state: State) -> Result<PgQueryResult, Error> {
        debug!("updating state for {} with state {:?}", id, state);
        let query = "update reminders set STATE = $1 where id = $2";
        sqlx::query(query)
            .bind(state)
            .bind(id)
            .execute(db).await
    }

    pub async fn get_active(db: &Pool<Postgres>) -> Result<Vec<Reminder>, Error> {
        debug!("get active reminders");
        sqlx::query_as("select * from reminders where state = $1")
            .bind(State::ACTIVE)
            .fetch_all(db).await
    }
    
    pub async fn get(db: &Pool<Postgres>, id: i32) -> Result<Reminder, Error> {
        debug!("get reminder for {}", id);
        sqlx::query_as("select * from reminders where id = $1")
            .bind(&id)
            .fetch_one(db).await
    }
}