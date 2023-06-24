use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use sqlx::{Pool, Postgres};
use crate::commands::slash_command::SlashCommand;

pub async fn call(ctx: &Context, aci: &ApplicationCommandInteraction, cmd: &dyn SlashCommand, database: &Pool<Postgres> ) {
    if let Err(why) = cmd.execute(ctx, aci, database)
        .await
    {
        warn!("Cannot respond to slash command '/{}': {}", cmd.name(), why);
    }
}