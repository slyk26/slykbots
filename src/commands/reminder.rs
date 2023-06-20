use std::future::Future;
use chrono::Duration;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption};
use serenity::model::prelude::command::CommandOptionType;
use sqlx::{Pool, Postgres};
use parse_duration::parse;
use serenity::model::prelude::{ChannelId, UserId};
use serenity::utils::MessageBuilder;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use crate::commands::slash_command::SlashCommand;
use crate::models::State;
use crate::service::reminder_service::{ACTIVE_REMINDERS, ReminderService};

pub struct Reminder;

#[async_trait]
impl SlashCommand for Reminder {
    fn name(&self) -> String {
        "reminder".to_string()
    }

    fn description(&self) -> String {
        "reminds you about something in the future".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        let message: CreateApplicationCommandOption = CreateApplicationCommandOption::default()
            .name("reminder").kind(CommandOptionType::String).max_length(2000)
            .description("your reminder message").clone();
        let when: CreateApplicationCommandOption = CreateApplicationCommandOption::default()
            .name("in").kind(CommandOptionType::String)
            .description("time in the future 2h30min")
            .required(true).clone();

        command.name(self.name()).description(self.description())
            .add_option(when)
            .add_option(message)
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &Pool<Postgres>) -> Result<(), SerenityError> {
        let channel_id = command.channel_id;
        let user_id = command.user.id;
        let options = command.data.options.clone();
        let reminder = options.iter().find(|option| option.name.eq("reminder"));
        let time = options.iter().find(|option| option.name.eq("in")).unwrap().value.as_ref().unwrap().to_string();
        let mut message = String::from("");
        let mut r = String::from("Failed creating a reminder. Try again later.");

        if let Some(f) = reminder {
            message = f.value.as_ref().unwrap().to_string();
        }

        if let Ok(duration) = parse(time.as_str()) {
            let diff = Duration::seconds(duration.as_secs() as i64);
            let time = chrono::offset::Utc::now() + diff;

            match ReminderService::insert(database, crate::models::Reminder {
                id: 0,
                channel_id: channel_id.to_string(),
                user_id: user_id.to_string(),
                message: Some(message.clone()),
                remind_at: time,
                state: State::ACTIVE,
            }).await {
                Ok(id) => {
                    ACTIVE_REMINDERS.lock().await.insert(id, create_task(duration, send_response(ctx.clone(), channel_id, user_id, message.clone(), id, database.clone())));
                    r = format!("Created Reminder with id {}! You will be notified at: {} UTC time.\n\
                    You can abort anytime using `/abort {}`", id, time.format("%d.%m.%Y %H:%M:%S"), id);
                }
                Err(e) => { println!("{}", e) }
            }
        } else {
            r = format!("Invalid time specified. Try something like: 3hours 20 min");
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(r))
            }).await
    }
}

pub fn create_task(duration: std::time::Duration, f: impl Future<Output=()> + Send + 'static) -> JoinHandle<()> {
    task::spawn(async move {
        sleep(duration).await;
        f.await;
    })
}

pub async fn send_response(ctx: Context, channel_id: ChannelId, user_id: UserId, message: String, id: i32, db: Pool<Postgres>) {
    let http = ctx.http.clone();
    let content = MessageBuilder::new()
        .mention(&user_id)
        .push_bold("[REMINDER] ")
        .push(message)
        .build();

    let _ = channel_id.send_message(http, |m| {
        m.content(content)
    }).await;
    let _ = ReminderService::update_state(&db, id, State::DONE).await;
    ACTIVE_REMINDERS.lock().await.remove(&id);
}
