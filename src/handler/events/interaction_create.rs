use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use crate::types::{COMMANDS, DB};
use crate::handler::interactions::application_command;

pub async fn call(ctx: &Context, interaction: &Interaction, commands: &COMMANDS, database: &DB) {
    if interaction.guild_locale().is_some() {
        match interaction {

            // regular response (text) => returns the result of the called SlashCommand
            Interaction::ApplicationCommand(interaction) => {
                match commands.get(interaction.data.name.as_str()) {
                    Some(cmd) => application_command::call(ctx, interaction, cmd.as_ref(), database).await,
                    None => error!("invalid command issued: {:?}", interaction.data)
                }
            }

            _ => { warn!("unsupported interaction type: {:?}", interaction.kind()) }
        }
    } else {
        match interaction {
            Interaction::ApplicationCommand(interaction) => {
                let _ = interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("The bot only works in a Server"))
                    }).await;
            }

            _ => { warn!("unsupported interaction type: {:?}", interaction.kind()) }
        }
    }
}