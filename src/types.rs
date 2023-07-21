use std::collections::HashMap;
use sqlx::{Pool, Postgres};
use crate::commands::SlashCommand;

pub type DB = Pool<Postgres>;
pub type COMMAND = dyn SlashCommand;
pub type COMMANDS = HashMap<String, Box<COMMAND>>;
