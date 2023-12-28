use log::info;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shared::serenity_utils::types::COMMANDS;
use crate::reddit::{post_thread, fetch_thread};

pub async fn call(c: &Context, ready: &Ready, _: &COMMANDS) {
    info!("{} is online!", ready.user.name);

    fetch_thread().await;
    post_thread(c.http.clone()).await;
}

