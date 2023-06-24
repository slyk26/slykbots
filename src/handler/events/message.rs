use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::http::Http;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use sqlx::{Pool, Postgres};
use crate::markov_chains::MarkovService;

pub async fn call(ctx: &Context, new_message: Message, db: &Pool<Postgres>) {
    if !mentions_bot(ctx.http.clone(), &new_message).await {
        MarkovService::destruct_message(&new_message, db).await;
    }

    // 7% chance after msg or always on @mention
    if thread_rng().gen_range(0..100) < 7 || mentions_bot(ctx.http.clone(), &new_message).await {
        MarkovService::send_message(ctx, &new_message, db).await;
    }
}

async fn mentions_bot(cache: Arc<Http>, msg: &Message) -> bool {
    match msg.mentions_me(cache).await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}
