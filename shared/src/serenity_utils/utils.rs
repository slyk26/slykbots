use std::fmt::Display;
use std::sync::Arc;
use log::error;
use serenity::http::{CacheHttp, Http};
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use songbird::Songbird;

pub async fn say(channel: ChannelId, http: impl AsRef<Http> + CacheHttp, msg: impl Display + Into<String>) {
    if let Err(e) = channel.say(http, msg).await {
        error!("cannot say: {}", e);
    }
}

pub async fn reply(d_message: &Message, http: impl CacheHttp, msg: impl Display + Into<String>) {
    if let Err(e) = d_message.reply(http, msg).await {
        error!("cannot reply: {}", e);
    }
}

pub async fn get_voicemanager(ctx: &Context) -> Arc<Songbird> {
    match songbird::get(ctx).await {
        Some(birb) => birb.clone(),
        None => panic!("Songbird is not registered")
    }
}
