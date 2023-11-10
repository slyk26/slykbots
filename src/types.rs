use std::collections::HashMap;
use crate::commands::SlashCommand;

pub type COMMAND = dyn SlashCommand;
pub type COMMANDS = HashMap<String, Box<COMMAND>>;