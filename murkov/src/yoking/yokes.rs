use std::time::Duration;
use rand::{thread_rng, Rng};
use serenity::{framework::standard::{macros::{command, group}, CommandResult}, prelude::Context, model::{prelude::{Message, UserId}, user::OnlineStatus}, Error};
use serenity::framework::standard::DispatchError;
use serenity::framework::standard::macros::hook;
use serenity::model::id::GuildId;

use crate::utils::{reply, say};

#[group]
#[commands(ping)]
pub struct Yokes;

#[command]
#[only_in(guilds)]
#[bucket = "ping"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("hello!");
    
    let online_users = get_online_members(ctx, msg.guild_id.unwrap()).await?;
    debug!("{:?}", online_users);
    let random_user = online_users.get(thread_rng().gen_range(0..online_users.len())).unwrap();
    
    say(msg.channel_id, &ctx.http, format!("<@{}>", random_user.0)).await;
    
    Ok(())
}


async fn get_online_members(ctx: &Context, guild_id: GuildId) -> Result<Vec<UserId>, Error> {
     let presences = guild_id.to_guild_cached(&ctx.cache).unwrap().presences;

     debug!("{:?}", presences);
     
     Ok(presences.iter()
         .filter_map(|(user_id, presence)| {
             match OnlineStatus::Online == presence.status || OnlineStatus::DoNotDisturb == presence.status {
                 true => Some(*user_id),
                 false => None,
             }
         })
         .collect::<Vec<UserId>>())
}

#[hook]
pub async fn dispatch_error_hook(ctx: &Context, msg: &Message, error: DispatchError, command_name: &str) {
    match error {
        DispatchError::Ratelimited(info) => {
           reply(msg, &ctx.http, format!("you can use that command in {} again", format_duration(info.rate_limit))).await;
        },
        _ => warn!("Unhandled dispatch error in {}.", command_name),
    }
}

fn format_duration(d: Duration) -> String {
    let total_seconds = d.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}min {:02}s", minutes, seconds)
}