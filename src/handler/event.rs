use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::VoiceState;
use serenity::prelude::*;
use crate::commands::{SlashCommand, Stats, Music};
use crate::handler::events::ready;
use crate::handler::events::interaction_create;
use crate::handler::events::message;
use crate::handler::events::voice_state_update;
use crate::types::{COMMANDS, DB};

pub struct EventHandler {
    pub database: DB,
    pub commands: COMMANDS
}

impl EventHandler {
    pub fn init(database:  DB) -> Self {
        let mut commands: COMMANDS = COMMANDS::new();
        commands.insert(Stats.name(), Box::new(Stats));
        commands.insert(Music.name(), Box::new(Music));

        EventHandler {database, commands}
    }
}

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        message::call(&ctx, new_message.clone(), &self.database).await;
    }

    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy, &self.commands).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        voice_state_update::call(ctx, old, new).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::call(&ctx, &interaction, &self.commands, &self.database).await;
    }
}
