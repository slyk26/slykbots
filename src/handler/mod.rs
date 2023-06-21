mod events;
mod interactions;

use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use sqlx::Postgres;
use crate::handler::events::ready;
use crate::handler::events::interaction_create;
use crate::handler::events::message;

pub struct Handler {
    pub database: sqlx::Pool<Postgres>
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        message::call(&ctx, new_message.clone(), &self.database).await;
    }

    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy, &self.database).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::call(&ctx, &interaction, &self.database).await;
    }
}