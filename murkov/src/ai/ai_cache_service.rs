use std::env::var;
use serenity::model::prelude::ChannelId;
use sqlx::{Error, query, query_as, Row};
use crate::ai::ai_cache_model::AiCacheModel;
use crate::PG;

pub struct AiCacheService;

impl AiCacheService {
    pub async fn get_prompt_history(channel_id: ChannelId) -> Vec<AiCacheModel> {
        debug!("getting prompts for: {}", channel_id);

        match query_as::<_, AiCacheModel>("select * from ai_chat_cache where channel_id = $1 order by id asc")
            .bind(channel_id.0 as i64)
            .fetch_all(PG.get().unwrap()).await {
            Ok(prompts) => prompts,
            Err(e) => {
                warn!("no prompts found for {} - {}", channel_id.0, e);
                vec![]
            }
        }
    }

    pub async fn update_prompt_history(channel_id: ChannelId, prompt: String) -> Result<i32, Error> {
        if let Err(e) = AiCacheService::insert_new_prompt(channel_id, prompt).await {
            error!("cannot insert new prompt: {}", e);
        }

        match AiCacheService::delete_oldest_prompt(channel_id).await {
            Ok(code) => Ok(code),
            Err(e) => {
                error!("cannot update prompt history: {}", e);
                Err(e)
            }
        }
    }

    async fn insert_new_prompt(channel_id: ChannelId, prompt: String) -> Result<i32, Error> {
        debug!("inserting prompt from - {} - with prompt - {} -", channel_id.0, prompt);

        match query("insert into ai_chat_cache(channel_id, prompt) values($1,$2) returning id")
            .bind(channel_id.0 as i64)
            .bind(prompt)
            .fetch_one(PG.get().unwrap()).await {
            Ok(row) => Ok(row.get::<i32, _>(0)),
            Err(e) => Err(e)
        }
    }

    async fn delete_oldest_prompt(channel_id: ChannelId) -> Result<i32, Error> {
        let count = AiCacheService::get_prompt_count(channel_id).await;

        if count <= var("MAX_USER_PROMPTS").expect("No user prompts specified in env").parse::<i64>().unwrap(){
            return Ok(-1);
        }

        match query("delete from ai_chat_cache where id = (select min(id) from ai_chat_cache where channel_id = $1)")
            .bind(channel_id.0 as i64)
            .execute(PG.get().unwrap()).await {
            Ok(_) => Ok(0),
            Err(e) => {
                warn!("cannot delete oldest prompt of {} reason: {}", channel_id.0, e);
                Ok(-2)
            }
        }
    }

    async fn get_prompt_count(channel_id: ChannelId) -> i64 {
        match query("select count(*) from ai_chat_cache where channel_id = $1")
            .bind(channel_id.0 as i64)
            .fetch_one(PG.get().unwrap()).await {
            Ok(row) => row.get::<i64, _>(0),
            Err(e) => {
                warn!("get_prompt_count: {}", e);
                -1
            }
        }
    }
}