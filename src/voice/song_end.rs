use std::sync::Arc;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use songbird::{Event, EventContext, EventHandler};
use EventHandler as VoiceEventHandler;
use crate::util::check_msg;


pub struct SongEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.chan_id
                .say(&self.http, "Song faded out completely!")
                .await,
        );

        None
    }
}