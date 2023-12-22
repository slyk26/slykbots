mod reddit;
mod handler;
mod subscriptions;

use std::env::var;
use std::sync::OnceLock;
use std::time::Duration;
use log::error;
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::prelude::GatewayIntents;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::handler::EventHandler;

static PG: OnceLock<Pool<Postgres>> = OnceLock::new();

#[tokio::main]
async fn main() {
    // load configs
    pretty_env_logger::init();
    let url = var("DATABASE_URL").expect("DATABASE_URL not found");
    let token = var("RPP_TOKEN").expect("RPP_TOKEN not found");
    let pool = PgPoolOptions::new()
        .max_lifetime(Duration::from_secs(10))
        .max_connections(25)
        .connect(url.as_str()).await.expect("Cannot create Database Pool");


    // create bot
    let mut bot = Client::builder(token.clone(), GatewayIntents::GUILDS)
        .event_handler(EventHandler::init())
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    PG.set(pool).expect("Error setting DB Pool");


    // start bot
    if let Err(why) = bot.start().await {
        error!("Client error: {:?}", why);
    }

}