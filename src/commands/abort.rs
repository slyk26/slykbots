use std::str::FromStr;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::slash_command::SlashCommand;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::UserId;
use sqlx::{Pool, Postgres};
use crate::models::State;
use crate::service::reminder_service::{ACTIVE_REMINDERS, ReminderService};

pub struct Abort;

#[async_trait]
impl SlashCommand for Abort {
    fn name(&self) -> String {
        "abort".to_string()
    }

    fn description(&self) -> String {
        "abort an existing reminder with the reminder id".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        let id: CreateApplicationCommandOption = CreateApplicationCommandOption::default()
            .name("id")
            .description("id of the reminder")
            .kind(CommandOptionType::Integer)
            .required(true).clone();

        command.name(self.name()).description(self.description())
            .add_option(id)
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction, database: &Pool<Postgres>) -> Result<(), SerenityError> {
        let options = command.data.options.clone();
        let id = options.iter().find(|option| option.name.eq("id")).unwrap().value.as_ref().unwrap().to_string().parse::<i32>();
        let mut ephemeral = true;
        let text;
        if let Ok(i) = id {
            if let Ok(reminder) = ReminderService::get(database, i).await {
                if let Ok(stored_id) = UserId::from_str(reminder.user_id.as_str()) {
                    if stored_id == command.user.id {
                        if reminder.state == State::ACTIVE {
                            text = "Aborted Reminder successfully!";
                            ephemeral = false;
                            ACTIVE_REMINDERS.lock().await.remove(&i);
                            let _ = ReminderService::update_state(&database, i, State::ABORTED).await;
                        } else {
                            text = "This reminder is not active anymore";
                        }
                    } else {
                        text = "That reminder does not belong to you.";
                    }
                } else {
                    text = "Parse Error on stored user id";
                }
            } else {
                text = "Id does not exist";
            }
        } else {
            text = "Abort failed: could not parse id from command";
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(text).ephemeral(ephemeral))
            }).await
    }
}
