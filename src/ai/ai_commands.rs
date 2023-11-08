use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, CreateCompletionRequestArgs};
use lazy_static::lazy_static;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::utils::{reply, say};

lazy_static! {
    static ref CLIENT: Client<OpenAIConfig> = Client::new();
}

#[group]
#[commands(ask)]
struct Ai;


#[command]
#[only_in(guilds)]
async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    let r = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-instruct")
        .prompt(&msg.content)
        .max_tokens(100_u16)
        .build()
        .unwrap();

    let response = match CLIENT
        .completions()
        .create(r).await {
        Ok(r) => r.choices.first().unwrap().text.clone(),
        Err(e) => {
            error!("{}", format!("OpenAI Error: {}", e));
            String::from("brain stopped working, try again")
        }
    };

    reply(msg, &ctx.http, response).await;
    Ok(())
}

// TODO: cache last 20 messages in db for better chatting experience
#[hook]
pub async fn dm_chatting(ctx: &Context, msg: &Message) {
    if !msg.is_private() { return; }

    let r = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .content(msg.content.clone())
                .build().unwrap(),
        ])
        .max_tokens(200_u16)
        .build()
        .unwrap();

    let response = match CLIENT
        .chat()
        .create(r).await {
        Ok(r) => r.choices.first().unwrap().message.content.as_ref().unwrap().clone(),
        Err(e) => {
            error!("{}", format!("OpenAI Error: {}", e));
            String::from("brain stopped working, try again")
        }
    };

    say(msg.channel_id, &ctx.http, response).await;
}