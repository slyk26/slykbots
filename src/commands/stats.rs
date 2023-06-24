use serenity::client::Context;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::utils::Color;
use sqlx::{Pool, Postgres};
use crate::commands::slash_command::SlashCommand;
use crate::markov_chains::MarkovService;

pub struct Stats;

#[async_trait]
impl SlashCommand for Stats {
    fn name(&self) -> String {
        "stats".to_string()
    }

    fn description(&self) -> String {
        "shows stats about this server".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &Pool<Postgres>) -> Result<(), SerenityError> {
        let mut embed = CreateEmbed::default();
        let guild_str = command.guild_id.unwrap().to_string();
        let (entries, used) = MarkovService::get_stats(database, &guild_str).await;

        embed.title("Stats")
            .colour(Color::from_rgb(255, 255, 255))
            .field(format!("learned {} Markov entries here", entries), "", false)
            .field(format!("learned enough words to talk: {}", MarkovService::get_max(database, &guild_str).await > 1000), "", false)
            .field(format!("active in {} servers", used), "", false)
            .field("made by: slyk26", "", false)
            .field(format!("version: {}", env!("CARGO_PKG_VERSION")), "", false);

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.add_embed(embed))
            }).await
    }
}
