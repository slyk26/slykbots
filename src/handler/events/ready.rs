use std::str::FromStr;
use std::time::Duration;
use serenity::model::application::command::Command;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, UserId};
use serenity::prelude::*;
use sqlx::{Pool, Postgres};
use tokio::time::interval;
use crate::commands::{COMMANDS, create_task, send_response};
use crate::service::reminder_service::{ACTIVE_REMINDERS, ReminderService};

pub async fn call(ctx: &Context, ready: &Ready, db: &Pool<Postgres>) {
    println!("{} is online!", ready.user.name);

    register_commands(&ctx).await;
    status_update_thread(ctx.clone());
    log_active_reminders();
    recover_reminder_tasks(&ctx, db).await;
}

fn log_active_reminders() {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            println!("{:?}", ACTIVE_REMINDERS.lock().await.keys());
        }
    });
}

fn status_update_thread(ctx_for_thread: Context) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::watching("forsen")).await;
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::listening("/help")).await;
        }
    });
}

async fn register_commands(ctx: &Context) {
    for cmd in COMMANDS.iter() {
        let result = Command::create_global_application_command(&ctx.http, |command| {
            cmd.1.register(command)
        }).await;

        let _ = match result {
            Ok(_) => println!("/{} registered", cmd.0),
            Err(e) => {
                println!("{:?}", e.to_string());
                panic!("Problem creating command")
            }
        };
    }
}

async fn recover_reminder_tasks(ctx: &Context, db: &Pool<Postgres>) {
    match ReminderService::get_active(&db).await {
        Ok(reminders) => {
            for r in reminders {
                let leftover = r.remind_at - chrono::Utc::now();
                let l: Duration = Duration::from_secs(leftover.num_seconds() as u64);
                let response = send_response(ctx.clone(), ChannelId::from_str(r.channel_id.as_str()).unwrap(),
                                      UserId::from_str(r.user_id.as_str()).unwrap(), r.message.unwrap_or(String::from("")), r.id, db.clone());

                if leftover < chrono::Duration::seconds(0) {
                    response.await;
                } else {
                    ACTIVE_REMINDERS.lock().await.insert(r.id, create_task(l, response));
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            // log could not fetch active reminders
        }
    }
}