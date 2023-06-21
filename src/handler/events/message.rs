use serenity::model::prelude::Message;
use serenity::prelude::Context;
use sqlx::{Pool, Postgres};
use crate::models::MarkovModel;
use crate::service::markov_service::MarkovService;

pub async fn call(_ctx: &Context, new_message: Message, db: &Pool<Postgres>) {
    if !new_message.author.bot {
        let words = new_message.content.split(' ').collect::<Vec<&str>>();

        for i in 0..words.len() {
            let current = words[i].to_string();
            let mut next: Option<String> = None;
            if i + 1 < words.len() {
                next = Some(words[i + 1].to_string());
            }
            MarkovService::process(db, MarkovModel{
                id: 0,
                guild_id: new_message.guild_id.unwrap().to_string(),
                current_word: current,
                next_word: next,
                frequency: 1,
            }).await;
        }
    }
}