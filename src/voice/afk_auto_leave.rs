use std::sync::Arc;
use serenity::async_trait;
use songbird::{EventContext, Songbird};
use songbird::Event;
use songbird::EventHandler;
use EventHandler as VoiceEventHandler;
use serenity::model::prelude::GuildId;

pub struct AfkAutoLeave {
    pub guild_id: GuildId,
    pub manager: Arc<Songbird>,
}

#[async_trait]
impl VoiceEventHandler for AfkAutoLeave {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if let Some(handler_lock) = self.manager.get(self.guild_id) {
            let mut handler = handler_lock.lock().await;
            debug!("{:?}", handler.queue().current_queue());
            if handler.queue().current_queue().is_empty() {
                let _dc = handler.leave().await;
            }
        };
        None
    }
}