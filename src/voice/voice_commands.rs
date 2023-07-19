use std::ops::Add;
use std::time::Duration;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::Message;
use serenity::prelude::{Context, Mentionable};
use serenity::utils::Color;
use songbird::{Event, TrackEvent};
use songbird::input::{Metadata, Restartable};
use crate::util::{check_msg, LEGACY_CMD};
use crate::voice::afk_auto_leave::AfkAutoLeave;
use crate::voice::track_info::TrackInfoNotifier;

#[group]
#[commands(join, leave, play, skip, list, stop)]
struct General;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "move ur ass to a vc first :LULA:").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::Play),
            TrackInfoNotifier {
                chan_id,
                http: send_http.clone(),
            },
        );

        handle.add_global_event(
            Event::Periodic(Duration::from_secs(300), None),
            AfkAutoLeave {
                guild_id,
                manager: manager.clone()
            },
        )
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "baj baj").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                error!("Err starting source: {:?}", why);
                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);
                return Ok(());
            }
        };

        debug!("{:?}", source);
        handler.enqueue_source(source.into());
        debug!("{:?}", handler.queue().current_queue().first().unwrap().get_info().await);

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Added song to queue: `{}`", handler.queue().current().unwrap().metadata().title.clone().unwrap_or(String::new())),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Song skipped: {} in queue.", queue.len() - 1),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.stop();

        check_msg(msg.channel_id.say(&ctx.http, "Queue cleared.").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue().current_queue();

        debug!("{:?}", queue);

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                let embed = e.colour(Color::from_rgb(255, 0, 0)).title("Current Bangers");

                if let Some(current) = queue.first() {
                    let uuid = current.uuid();
                    for track in queue {
                        let (name, value) = format_track(track.uuid().eq(&uuid), track.metadata());
                        embed.field(name, value, false);
                    }
                } else {
                    embed.field("No songs in queue", format!("add some with {}play `url` !", LEGACY_CMD), false);
                }

                embed
            })
        }).await;
    }

    Ok(())
}

fn format_track(first: bool, m: &Metadata) -> (String, String) {
    let mut top = String::new();
    if first {
        top = String::from("👉 ");
    }
    top = top.add(m.title.clone().unwrap_or(String::new()).as_str());
    let bottom =
        format!("by {}", m.artist.clone().unwrap());

    (top, bottom)
}