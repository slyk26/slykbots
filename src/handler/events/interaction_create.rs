use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use sqlx::{Pool, Postgres};
use crate::commands::COMMANDS;
use crate::handler::interactions::application_command;

pub async fn call(ctx: &Context, interaction: &Interaction, database: &Pool<Postgres>) {
    if interaction.guild_locale().is_some() {
        match interaction {

            // regular response (text) => returns the result of the called SlashCommand
            Interaction::ApplicationCommand(interaction) => {
                let cmd = COMMANDS.get(interaction.data.name.as_str()).expect("No Command found in command map");
                application_command::call(ctx, interaction, cmd.as_ref(), database).await;
            }

            _ => { warn!("unknown interaction type") }
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

            _ => { warn!("unknown interaction type") }
        }
    }
}