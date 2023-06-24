use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct MarkovModel {
    pub id: i32,
    pub guild_id: String,
    pub current_word: String,
    pub next_word: Option<String>,
    pub frequency: i32,
}