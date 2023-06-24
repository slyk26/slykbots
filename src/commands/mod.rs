mod stats;

use lazy_static::lazy_static;
use std::collections::HashMap;
use slash_command::SlashCommand;

use crate::commands::stats::Stats;

lazy_static! {
    pub static ref COMMANDS: HashMap<String, Box<dyn SlashCommand>> = {
        let mut m: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        m.insert(Stats.name(), Box::new(Stats));
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