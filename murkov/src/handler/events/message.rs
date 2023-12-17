use std::sync::Arc;
use log::error;
use rand::{Rng, thread_rng};
use serenity::http::Http;
use serenity::model::prelude::{Message};
use serenity::prelude::Context;
use crate::markov_chains::MarkovService;
use crate::LEGACY_CMD;
use crate::settings::{MARKOV_SETTING, SettingsService};

pub async fn call(ctx: &Context, new_message: Message) {
    let cache = ctx.http.clone();

    if !new_message.is_private() {
        if SettingsService::is_enabled(new_message.guild_id.unwrap().0 as i64, MARKOV_SETTING.to_string()).await {
            if !mentions_bot(&cache, &new_message).await && !is_legacy_command(&new_message) {
                MarkovService::destruct_message(&new_message).await;
            }

            // 7% chance after msg or always on @mention
            if (thread_rng().gen_range(0..100) < 7 || mentions_bot(&cache, &new_message).await) && !is_legacy_command(&new_message) {
                MarkovService::send_message(ctx, &new_message).await;
            }
        }
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
