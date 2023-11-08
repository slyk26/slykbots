use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateCompletionRequestArgs;
use lazy_static::lazy_static;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::utils::reply;

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