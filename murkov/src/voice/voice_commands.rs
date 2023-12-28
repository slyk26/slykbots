use std::collections::HashMap;
use std::ops::{AddAssign, Add};
use std::time::Duration;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::{Message, UserId, VoiceState};
use serenity::prelude::{Context, Mentionable};
use serenity::utils::Color;
use songbird::tracks::TrackHandle;
use songbird::{Event, TrackEvent};
use songbird::input::{Metadata, Restartable};
use url::Url;
use shared::serenity_utils::{get_voicemanager, reply, say};
use crate::voice::afk_auto_leave::AfkAutoLeave;
use crate::voice::track_info::TrackInfoNotifier;
use crate::LEGACY_CMD;
use crate::settings::{MUSIC_SETTING, SettingsService};

#[group]
#[commands(join, leave, play, skip, list, stop, info, remove)]
struct Voice;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    if !SettingsService::is_enabled(msg.guild_id.unwrap().0 as i64, MUSIC_SETTING.to_string()).await { return Ok(()); }
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let chan_id = msg.channel_id;
    let manager = get_voicemanager(ctx).await;

    let was_in_vc = manager.get(guild_id).is_some();

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            reply(msg, &ctx.http, "move ur ass to a vc first LULE").await;
            return Ok(());
        }
    };

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if success.is_ok() {
        let send_http = ctx.http.clone();
        let mut handle = handle_lock.lock().await;

        if !was_in_vc {
            handle.add_global_event(Event::Track(TrackEvent::Play), TrackInfoNotifier { chan_id, http: send_http.clone() });
            handle.add_global_event(Event::Periodic(Duration::from_secs(300), None), AfkAutoLeave { guild_id, manager: manager.clone() });
        }
        say(chan_id, &ctx.http, format!("{} 👀", connect_to.mention())).await;
    } else {
        say(chan_id, &ctx.http, format!("MODS why can't I join {}? angrE", connect_to.mention())).await;
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = get_voicemanager(ctx).await;
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            reply(msg, &ctx.http, e.to_string()).await;
        }
        reply(msg, &ctx.http, "baj baj").await;
    } else {
        reply(msg, &ctx.http, "let me lurk in peace man").await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let chan_id = msg.channel_id;
    let manager = get_voicemanager(ctx).await;

    debug!("{:?}", guild.voice_states);

    if is_in_vc(guild.voice_states, msg.author.id, ctx.http.get_current_user().await.unwrap().id) {
        if let Some(handler_lock) = manager.get(guild_id) {
            let url = match args.single::<String>() {
                Ok(url) => url,
                Err(_) => {
                    say(msg.channel_id, &ctx.http, "buckeroo give me a link or words to search").await;
                    return Ok(());
                }
            };

            let mut handler = handler_lock.lock().await;
            let source: Option<Restartable> = if Url::parse(&url).is_ok() {
                url_source(url).await
            } else {
                word_source(args.rewind()).await
            };

            if source.is_none() {
                say(chan_id, &ctx.http, "watafak I failed using youtube").await;
                return Ok(());
            };

            debug!("{:?}", source);
            handler.enqueue_source(source.unwrap().into());
            debug!("{:?}", handler.queue().current_queue().first().unwrap().get_info().await);

            reply(msg, &ctx.http,
                  format!("Added song to queue: `{}`", handler.queue().current_queue().last().unwrap().metadata().title.clone().unwrap_or(String::new()))).await;
        } else {
            reply(msg, &ctx.http, "let me lurk in peace madgE").await;
        }
    } else {
        reply(msg, &ctx.http, "join vc first LULE").await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = get_voicemanager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let how_many = args.single::<i32>().unwrap_or(1);

        queue.pause()?;

        queue.modify_queue(|q| {
            for _ in 0..how_many {
                q.pop_front();
            }
        });

        queue.resume()?;

        reply(msg, &ctx.http, format!("{} Song(s) skipped! 👉 {} in queue.", how_many, queue.len())).await;
    } else {
        reply(msg, &ctx.http, "let me lurk in peace madgE").await;
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = get_voicemanager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();

        reply(msg, &ctx.http, "Queue cleared").await;
    } else {
        reply(msg, &ctx.http, "let me lurk in peace madgE").await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = get_voicemanager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue().current_queue();

        debug!("{:?}", queue);

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                let total: u64 = queue.iter().map(|track| track.metadata().duration.unwrap_or(Duration::from_secs(0)).as_secs()).sum();
                let embed = e.colour(Color::from_rgb(255, 0, 0)).title(format!("Current Bangers - Total: ({})", format_duration(&Duration::from_secs(total))));

                if let Some(current) = queue.first() {
                    let uuid = current.uuid();
                    for i in 0..queue.len() {
                        if i > 10 {
                            embed.field(format!("And {} more...", queue.len() - i), "", false);
                            break;
                        }
                        let track = queue.get(i).unwrap();
                        let (name, mut value) = format_track(track.uuid().eq(&uuid), track.metadata());
                        value = value.add(format!(" - ({})", format_duration(&track.metadata().duration.unwrap())).as_str());
                        embed.field(name, value, false);
                    }
                } else {
                    embed.field("No songs in queue", format!("add some with {}play!", LEGACY_CMD), false);
                }

                embed
            })
        }).await;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = get_voicemanager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue().current_queue();
        let current_song: &TrackHandle = queue.first().unwrap();
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                let m = current_song.metadata();
                let embed = e.colour(Color::from_rgb(255, 0, 0)).title(m.title.as_ref().unwrap().to_string())
                    .image(m.thumbnail.as_ref().unwrap())
                    .field("Channel", format!("{} - [Link]({})", m.artist.as_ref().unwrap_or(&"unknown".to_string()), m.source_url.as_ref().unwrap_or(&"".to_string())), false)
                    .field("Length", format_duration(&m.duration.unwrap()), true)
                    .footer(|f|
                        f.icon_url("https://www.youtube.com/s/desktop/1f2ae858/img/favicon_48x48.png")
                            .text(format!(" x slykbots - v{}", env!("CARGO_PKG_VERSION")))
                    );
                embed
            })
        }).await;
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let how_many = args.single::<usize>().unwrap_or(1);
    let guild_id = msg.guild(&ctx.cache).unwrap().id;
    let manager = get_voicemanager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let current_song = queue.current().unwrap();
        debug!("{:?}", current_song);

        if how_many >= queue.len() {
            debug!("stop because {how_many} >= queue size");
            queue.stop();
        } else {
            queue.modify_queue(|q| {
                for i in 0..how_many {
                    debug!("loop {i}");
                    q.pop_back();
                }
            });
        }
    }

    reply(msg, &ctx.http, "removed ").await;
    Ok(())
}

fn format_duration(d: &Duration) -> String {
    let total_seconds = d.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn format_track(first: bool, m: &Metadata) -> (String, String) {
    let mut top = String::new();
    if first {
        top = String::from("👉 ");
    }

    top.add_assign(m.title.clone().unwrap_or_default().as_str());
    let bottom =
        format!("by {}", m.artist.clone().unwrap());

    (top, bottom)
}

async fn url_source(url: String) -> Option<Restartable> {
    match Restartable::ytdl(url, true).await {
        Ok(source) => Some(source),
        Err(why) => {
            error!("(play) error from URL: {:?}", why);
            None
        }
    }
}

async fn word_source(args: &mut Args) -> Option<Restartable> {
    let mut query = String::new();
    for arg in args.iter::<String>() {
        query.add_assign(&*arg.unwrap());
    }

    match Restartable::ytdl_search(query, true).await {
        Ok(source) => Some(source),
        Err(why) => {
            error!("(play) error with words: {:?}, {:?}", args, why);
            None
        }
    }
}

fn is_in_vc(voice_states: HashMap<UserId, VoiceState>, user: UserId, bot: UserId) -> bool {
    if voice_states.contains_key(&user) && voice_states.contains_key(&bot) {
        return voice_states.get(&user).unwrap().channel_id.unwrap() == voice_states.get(&bot).unwrap().channel_id.unwrap();
    }
    false
}