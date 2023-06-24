mod commands;
mod handler;
mod markov_chains;

#[macro_use]
extern crate log;
use std::env::var;
use std::time::Duration;
use dotenv::dotenv;
use serenity::prelude;
use serenity::prelude::GatewayIntents;
use crate::handler::Handler;

#[tokio::main]
async fn main() {
    // load configs
    dotenv().ok();
    pretty_env_logger::init();
    let url = var("DATABASE_URL").expect("DATABASE_URL not found");
    let token = var("BOT_TOKEN").expect("BOT_TOKEN not found");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_lifetime(Duration::from_secs(60))
        .max_connections(25)
        .connect(url.as_str()).await.expect("Cannot create Database Pool");
    if let Err(e) = sqlx::migrate!().run(&pool).await {
        error!("Migration: {:?}",e);
    }
    // create bot
    let mut bot = prelude::Client::builder(token, GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES)
        .event_handler(Handler { database: pool })
        .await
        .expect("Error creating client");

    // start bot
    if let Err(why) = bot.start().await {
        error!("Client error: {:?}", why);
    }
}