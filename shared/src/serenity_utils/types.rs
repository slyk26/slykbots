use std::collections::HashMap;
use crate::serenity_utils::SlashCommand;

pub type COMMAND = dyn SlashCommand;
pub type COMMANDS = HashMap<String, Box<COMMAND>>;