use sqlx::FromRow;

pub const MARKOV_SETTING: &str = "module.markov.enabled";
pub const AI_SETTING: &str = "module.ai.enabled";
pub const MUSIC_SETTING: &str = "module.music.enabled";

#[derive(Debug, FromRow, Clone)]
pub struct Setting {
    pub id: i32,
    pub guild_id: i64,
    pub setting: String,
    pub val: Option<String>
}