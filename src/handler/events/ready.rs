use serenity::model::application::command::Command;
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;
use crate::types::COMMANDS;

pub async fn call(ctx: &Context, ready: &Ready, commands: &COMMANDS) {
    info!("{} is online!", ready.user.name);

    register_commands(ctx, commands).await;
    ctx.set_activity(Activity::playing("with AI")).await;
}

async fn register_commands(ctx: &Context, commands: &COMMANDS) {
    for cmd in commands.iter() {
        let result = Command::create_global_application_command(&ctx.http, |command| {
            cmd.1.register(command)
        }).await;

        match result {
            Ok(_) => info!("/{} registered", cmd.0),
            Err(e) => {
                error!("Error creating command: {}", e)
            }
        };
    }
}