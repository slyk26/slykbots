use std::sync::Arc;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use songbird::EventContext;
use songbird::Event;
use songbird::EventHandler;
use EventHandler as VoiceEventHandler;
use crate::voice::voice_utils::say;

pub struct TrackInfoNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackInfoNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let (_, handle) = track_list.first().unwrap();
            say(self.chan_id, &self.http, &format!("🎧 Now playing: `{}`", handle.metadata().title.clone().unwrap())).await;
        }
        None
    }
}