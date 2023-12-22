use std::collections::HashMap;
use std::sync::Arc;
use log::info;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shared::serenity_utils::types::COMMANDS;
use crate::reddit::{RedditPost, subreddit_threads};

pub async fn call(c: &Context, ready: &Ready, _: &COMMANDS) {
    info!("{} is online!", ready.user.name);

    let cache: Arc<Mutex<HashMap<String, Vec<RedditPost>>>> = Arc::new(Mutex::new(HashMap::new()));

    subreddit_threads(cache.clone(), c.http.clone()).await;
}

