use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::types::{COMMAND, DB};

pub async fn call(ctx: &Context, aci: &ApplicationCommandInteraction, cmd: &COMMAND, database: &DB ) {
    if let Err(why) = cmd.execute(ctx, aci, database)
        .await
    {
        warn!("Cannot respond to slash command '/{}': {}", cmd.name(), why);
    }
}