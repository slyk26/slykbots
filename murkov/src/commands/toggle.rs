use std::env;
use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::SerenityError;
use crate::settings::{AI_SETTING, MARKOV_SETTING, MUSIC_SETTING, SettingsService};
use super::SlashCommand;

pub struct Toggle;

#[async_trait]
impl SlashCommand for Toggle {
    fn name (&self) -> String {
        "toggle".to_string()
    }
    
    fn description(&self) -> String {
        "enables/disables a module for the server".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        let module: CreateApplicationCommandOption = CreateApplicationCommandOption::default()
            .name("module")
            .kind(CommandOptionType::String)
            .description("a feature of slykbots")
            .required(true)
            .add_string_choice("markov generation", MARKOV_SETTING)
            .add_string_choice("ai generation", AI_SETTING)
            .add_string_choice("music player", MUSIC_SETTING)
            .clone();

        let enabled: CreateApplicationCommandOption = CreateApplicationCommandOption::default()
            .name("enabled")
            .kind(CommandOptionType::Boolean)
            .description("enable or disable the feature")
            .required(true)
            .clone();

        command
            .name(self.name())
            .description(self.description())
            .add_option(module)
            .add_option(enabled)
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        if env::var("TOGGLE_DISABLED").unwrap_or("false".to_string()).parse::<bool>().unwrap_or(false) {
             return Ok(());
        }

        let options = &command.data.options;

        let module = options.iter().find(|o| o.name.eq("module")).unwrap().value.as_ref().unwrap().to_string().replace('\"',"");
        let enabled = options.iter().find(|o| o.name.eq("enabled")).unwrap().value.as_ref().unwrap().to_string();

        let mut msg = "updated successfuly";

        if let Err(e) = SettingsService::update_setting(command.guild_id.unwrap().0 as i64, module, enabled).await {
            error!("could not update setting: {e}");
            msg = "update failed - conduct slyk26"
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(msg))
            }).await
    }


}