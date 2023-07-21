mod stats;
mod music;

pub use crate::commands::stats::Stats;
pub use crate::commands::music::Music;
pub use slash_command::SlashCommand;

mod slash_command {
    use serenity::async_trait;
    use serenity::builder::{CreateApplicationCommand};
    use serenity::client::Context;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::prelude::SerenityError;
    use crate::types::DB;

    #[async_trait]
    pub trait SlashCommand: Send + Sync {
        fn name(&self) -> String;

        fn description(&self) -> String;

        fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
            command.name(self.name()).description(self.description())
        }

        async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &DB) -> Result<(), SerenityError>;
    }
}