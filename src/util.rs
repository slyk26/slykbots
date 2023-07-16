use std::collections::HashMap;
use sqlx::{Pool, Postgres};
use std::result::Result as SerenityResult;
use serenity::model::channel::Message;
use serenity::Error;
use crate::commands::SlashCommand;

pub const LEGACY_CMD: &str = ".";
pub type DB = Pool<Postgres>;
pub type COMMAND = dyn SlashCommand;
pub type COMMANDS = HashMap<String, Box<COMMAND>>;

pub fn check_msg(result: SerenityResult<Message, Error>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
