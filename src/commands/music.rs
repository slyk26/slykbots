use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::utils::Color;
use crate::commands::SlashCommand;
use crate::types::DB;
use crate::LEGACY_CMD;

pub struct Music;

#[async_trait]
impl SlashCommand for Music {
    fn name(&self) -> String {
       "music".to_string()
    }

    fn description(&self) -> String {
        "shows all related legacy commands for the music player".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, _database: &DB) -> Result<(), SerenityError> {
        let mut embed = CreateEmbed::default();
        embed.title("Youtube Player")
            .colour(Color::from_rgb(255, 0, 0))
            .field(format!("{}join", LEGACY_CMD), "joins the vc\n(you have to be in a vc first)", false)
            .field(format!("{}play", LEGACY_CMD), "play a url or use words to search", false)
            .field(format!("{}stop", LEGACY_CMD), "clears queue", false)
            .field(format!("{}leave", LEGACY_CMD), "removes bot from vc\n(also clears queue)", false)
            .field(format!("{}skip", LEGACY_CMD), "skip the current song", false)
            .field(format!("{}list", LEGACY_CMD), "show the queue", false)
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