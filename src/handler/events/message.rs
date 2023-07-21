use std::sync::Arc;
use rand::{Rng, thread_rng};
use serenity::http::Http;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::markov_chains::MarkovService;
use crate::types::DB;
use crate::voice::voice_utils::LEGACY_CMD;

pub async fn call(ctx: &Context, new_message: Message, db: &DB) {
    let cache = ctx.http.clone();

    if !mentions_bot(&cache, &new_message).await && !is_legacy_command(&new_message) {
        MarkovService::destruct_message(&new_message, db).await;
    }

    // 7% chance after msg or always on @mention
    if thread_rng().gen_range(0..100) < 7 || mentions_bot(&cache, &new_message).await {
        MarkovService::send_message(ctx, &new_message, db).await;
    }
}

async fn mentions_bot(cache: &Arc<Http>, msg: &Message) -> bool {
    match msg.mentions_me(cache).await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}

fn is_legacy_command(new_message: &Message) -> bool {
    new_message.content.starts_with(LEGACY_CMD)
}
