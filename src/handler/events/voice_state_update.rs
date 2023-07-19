use serenity::model::prelude::VoiceState;
use serenity::prelude::Context;

pub async fn call(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    leave_when_alone(ctx, old ,new).await;
}

async fn leave_when_alone(ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    let bot_id = ctx.http.get_current_user().await.unwrap().id;
    let last_user_id;
    let manager = songbird::get(&ctx).await.unwrap();


    if old.is_some() {
        last_user_id = old.unwrap().user_id;
    } else {
        last_user_id = new.user_id;
    }

    let mut vs = new.guild_id.unwrap().to_guild_cached(ctx.cache).unwrap().voice_states;
    vs.remove(&last_user_id);


    if vs.len() == 1 && vs.get(&bot_id).is_some() {
        if let Err(e) = manager.leave(new.guild_id.unwrap()).await {
            error!("{}", e);
        }
    }
}