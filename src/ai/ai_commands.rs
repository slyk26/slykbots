use std::env::var;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, CreateCompletionRequestArgs};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::http::Typing;
use serenity::model::prelude::{Message};
use serenity::prelude::Context;
use crate::AI;
use crate::ai::ai_cache_service::AiCacheService;
use crate::utils::{reply, say};

#[group]
#[commands(ask)]
struct Ai;


#[command]
#[only_in(guilds)]
async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    if !matches!(var("AI").unwrap_or(String::from("0")).as_str(), "1") { return Ok(()); }

    let r = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-instruct")
        .prompt(&msg.content)
        .max_tokens(100_u16)
        .build()
        .unwrap();

    let response = match AI.get().unwrap()
        .completions()
        .create(r).await {
        Ok(r) => r.choices.first().unwrap().text.clone(),
        Err(e) => {
            error!("{}", format!("OpenAI Error: {}", e));
            String::from("brain stopped working, try again later")
        }
    };

    reply(msg, &ctx.http, response).await;
    Ok(())
}

#[hook]
pub async fn dm_chatting(ctx: &Context, msg: &Message) {
    if !msg.is_private() || !matches!(var("AI").unwrap_or(String::from("0")).as_str(), "1") { return; }

    let typing = Typing::start(ctx.http.clone(), msg.channel_id.0).unwrap();
    let new_prompt = ChatCompletionRequestMessageArgs::default()
        .content(&msg.content)
        .build().unwrap();

    debug!("created new prompt");

    let mut prompts = AiCacheService::get_prompt_history(msg.channel_id).await.iter().map(|model| {
        ChatCompletionRequestMessageArgs::default()
            .content(&model.prompt)
            .build().unwrap()
    }).collect::<Vec<ChatCompletionRequestMessage>>();

    debug!("mapping of prompts done");
    prompts.push(new_prompt);
    debug!("asking api with: {:?}", prompts);

    let r = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(prompts)
        .max_tokens(300_u16)
        .build()
        .unwrap();

    debug!("creating request done");

    let response = match AI.get().unwrap()
        .chat()
        .create(r).await {
        Ok(r) => r.choices.first().unwrap().message.content.as_ref().unwrap().clone(),
        Err(e) => {
            error!("{}", format!("OpenAI Error: {}", e));
            String::from("brain stopped working, try again later")
        }
    };

    debug!("getting ai response");

    let result = match AiCacheService::update_prompt_history(msg.channel_id, msg.content.clone()).await {
        Ok(code) => String::from(match code {
            0 => "oldest prompt deleted successfully",
            -1 => "no delete on update",
            -2 => "delete failed",
            _ => "unknown error"
        }),
        Err(e) => e.to_string()
    };

    debug!("{}", result);

    say(msg.channel_id, &ctx.http, response).await;
    Typing::stop(typing);
}