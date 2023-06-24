use std::time::Duration;
use serenity::model::application::command::Command;
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;
use sqlx::{Pool, Postgres};
use tokio::time::interval;
use crate::commands::COMMANDS;

pub async fn call(ctx: &Context, ready: &Ready, _db: &Pool<Postgres>) {
    info!("{} is online!", ready.user.name);

    register_commands(ctx).await;
    status_update_thread(ctx.clone());
}

fn status_update_thread(ctx_for_thread: Context) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::watching("forsen")).await;
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::watching("/stats")).await;
        }
    });
}

async fn register_commands(ctx: &Context) {
    for cmd in COMMANDS.iter() {
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