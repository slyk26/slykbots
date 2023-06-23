use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use sqlx::{Pool, Postgres};
use crate::models::MarkovModel;
use crate::service::markov_service::MarkovService;

pub async fn call(ctx: &Context, new_message: Message, db: &Pool<Postgres>) {
    if ! mentions_me_wrapper(ctx.http.clone(), &new_message).await {
        destruct_message(&new_message, db).await;
    }

    if !new_message.author.bot &&
        (thread_rng().gen_range(0..100) < 7 || mentions_me_wrapper(ctx.http.clone(), &new_message).await) {
        send_message(ctx, new_message.channel_id, db).await;
    }
}

async fn mentions_me_wrapper(cache: Arc<Http>, msg: &Message) -> bool {
    match msg.mentions_me(cache).await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}

async fn send_message(ctx: &Context, channel: ChannelId, db: &Pool<Postgres>) {
    let message = MarkovService::generate_message(db).await;
    let _ = channel.send_message(&ctx.http, |m| {
        m.content(message)
    }).await;
}

async fn destruct_message(new_message: &Message, db: &Pool<Postgres>) {
    if !new_message.author.bot {
        let words = new_message.content.split(' ').collect::<Vec<&str>>();

        for i in 0..words.len() {
            let current = words[i].to_string();
            let mut next: Option<String> = None;
            if i + 1 < words.len() {
                next = Some(words[i + 1].to_string());
            }
            MarkovService::store(db, MarkovModel {
                id: 0,
                guild_id: new_message.guild_id.unwrap().to_string(),
                current_word: current,
                next_word: next,
                frequency: 1,
            }).await;
        }
    }
}