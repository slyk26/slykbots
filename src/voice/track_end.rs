use std::sync::Arc;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use songbird::EventContext;
use songbird::Event;
use crate::util::check_msg;
use songbird::EventHandler;
use EventHandler as VoiceEventHandler;

pub struct TrackEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            check_msg(
                self.chan_id
                    .say(&self.http, &format!("Tracks ended: {}.", track_list.len()))
                    .await,
            );
        }
        None
    }
}