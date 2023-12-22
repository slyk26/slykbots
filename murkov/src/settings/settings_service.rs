use log::debug;
use sqlx::{Error, query, query_as};

use crate::PG;

use super::Setting;

pub struct SettingsService;

impl SettingsService {
    pub async fn get_setting(guild_id: i64, key: String) -> Result<Setting, Error> {
        debug!("get setting for {guild_id}");
        
        query_as("select * from settings where guild_id = $1 and setting = $2")
        .bind(guild_id)
        .bind(key)
        .fetch_one(PG.get().unwrap()).await
    }

    pub async fn update_setting(guild_id: i64, key: String, value: String) -> Result<(), Error> {
        debug!("update {key} to {value}");

        match query("insert into settings(guild_id, setting, val) values($1,$2,$3) on conflict (guild_id, setting) do update set val = $3")
            .bind(guild_id)
            .bind(key)
            .bind(value)
            .execute(PG.get().unwrap()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub async fn is_enabled(guild_id: i64, key: String) -> bool {
        match SettingsService::get_setting(guild_id, key).await {
            Ok(s) => s.val.unwrap().parse::<bool>().unwrap_or(false),
            Err(e) => {
                warn!("error on checking boolean setting: {e}");
                false
            }
        }
    }
}