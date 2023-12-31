use serenity::model::prelude::VoiceState;
use serenity::prelude::Context;
use shared::serenity_utils::get_voicemanager;

pub async fn call(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    leave_when_alone(ctx, old ,new).await;
}

async fn leave_when_alone(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    let bot_id = ctx.http.get_current_user().await.unwrap().id;
    let last_user_id;
    let manager = get_voicemanager(&ctx).await;

    debug!("{:?}", old);
    debug!("{:?}", new);

    if let Some(old) = old {
        last_user_id = old.user_id;
    } else {
        last_user_id = new.user_id;
    };

    let mut vs = new.guild_id.unwrap().to_guild_cached(ctx.cache).unwrap().voice_states;
    vs.remove(&last_user_id);

    if vs.len() == 1 && vs.get(&bot_id).is_some() {
        if let Err(e) = manager.remove(new.guild_id.unwrap()).await {
            error!("{}", e);
        }
    }
}