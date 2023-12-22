use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::handler::events::ready;
use shared::serenity_utils::types::COMMANDS;

pub struct EventHandler {
    pub commands: COMMANDS
}

impl EventHandler {
    pub fn init() -> Self {
        let commands: COMMANDS = COMMANDS::new();

        EventHandler {commands}
    }
}

#[async_trait]
impl serenity::prelude::EventHandler for EventHandler {

    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy, &self.commands).await;
    }
}
