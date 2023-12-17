use log::warn;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use shared::serenity_utils::types::COMMAND;

pub async fn call(ctx: &Context, aci: &ApplicationCommandInteraction, cmd: &COMMAND ) {
    if let Err(why) = cmd.execute(ctx, aci)
        .await
    {
        warn!("Cannot respond to slash command '/{}': {}", cmd.name(), why);
    }
}