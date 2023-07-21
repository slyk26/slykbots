use serenity::client::Context;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::utils::Color;
use crate::commands::slash_command::SlashCommand;
use crate::markov_chains::MarkovService;
use crate::types::DB;

pub struct Stats;

#[async_trait]
impl SlashCommand for Stats {
    fn name(&self) -> String {
        "stats".to_string()
    }

    fn description(&self) -> String {
        "shows stats about this server".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &DB) -> Result<(), SerenityError> {
        let mut embed = CreateEmbed::default();
        let guild_str = command.guild_id.unwrap().to_string();
        let enough = MarkovService::get_max(database, &guild_str).await > 1000;
        let (entries, used) = MarkovService::get_stats(database, &guild_str).await;

        embed.title("Stats")
            .colour(Color::from_rgb(255, 255, 255))
            .field("Markov entries", entries, false)
            .field("can talk yet?", format!("{}", enough), false)
            .field("active servers", used, false)
            .field("========================", "", false)
            .footer(|f| {
                f.text(format!("by slyk26 \t\t\t\t v.{}", env!("CARGO_PKG_VERSION")))
            });

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.add_embed(embed))
            }).await
    }
}
