use log::error;
use sqlx::{FromRow, query_as};
use crate::PG;

pub struct SubService;

impl SubService {
    pub async fn get_all() -> Vec<Subscription> {
        match query_as("select * from rpp_subscriptions")
            .fetch_all(PG.get().unwrap()).await {
            Ok(rows) => rows,
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }
}

#[derive(Debug, FromRow, Clone)]
pub struct Subscription {
    pub id: i32,
    pub channel_id: i64,
    pub subreddit: String,
}