mod reminder;
mod help;
mod abort;

use lazy_static::lazy_static;
use std::collections::HashMap;
use slash_command::SlashCommand;
use reminder::Reminder;
use help::Help;
use crate::commands::abort::Abort;

pub use crate::commands::reminder::create_task;
pub use crate::commands::reminder::send_response;

lazy_static! {
    pub static ref COMMANDS: HashMap<String, Box<dyn SlashCommand>> = {
        let mut m: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        m.insert(Reminder.name(), Box::new(Reminder));
        m.insert(Help.name(), Box::new(Help));
        m.insert(Abort.name(), Box::new(Abort));
        m
    };
}

pub mod slash_command {
    use serenity::async_trait;
    use serenity::builder::{CreateApplicationCommand};
    use serenity::client::Context;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::prelude::SerenityError;
    use sqlx::{Pool, Postgres};

    #[async_trait]
    pub trait SlashCommand: Send + Sync {
        fn name(&self) -> String;

        fn description(&self) -> String;

        fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
            command.name(self.name()).description(self.description())
        }

        async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &Pool<Postgres>) -> Result<(), SerenityError>;
    }
}