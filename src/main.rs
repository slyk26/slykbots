mod commands;
mod handler;
mod markov_chains;
mod voice;
mod util;

#[macro_use]
extern crate log;

use std::env::var;
use std::time::Duration;
use serenity::prelude::{Client, GatewayIntents};
use sqlx::postgres::PgPoolOptions;
use songbird::SerenityInit;
use crate::handler::EventHandler;
use serenity::framework::StandardFramework;
use crate::util::LEGACY_CMD;
use crate::voice::GENERAL_GROUP;

#[tokio::main]
async fn main() {
    // load configs
    pretty_env_logger::init();
    let url = var("DATABASE_URL").expect("DATABASE_URL not found");
    let token = var("BOT_TOKEN").expect("BOT_TOKEN not found");
    let pool = PgPoolOptions::new()
        .max_lifetime(Duration::from_secs(10))
        .max_connections(25)
        .connect(url.as_str()).await.expect("Cannot create Database Pool");
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        error!("Migration: {:?}",e);
    }

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(LEGACY_CMD))
        .group(&GENERAL_GROUP);

    // create bot
    let mut bot = Client::builder(token.clone(),
                                  GatewayIntents::MESSAGE_CONTENT |
                                      GatewayIntents::GUILD_MESSAGES |
                                      GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES)
        .event_handler(EventHandler::init(pool))
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start bot
    if let Err(why) = bot.start().await {
        error!("Client error: {:?}", why);
    }
}