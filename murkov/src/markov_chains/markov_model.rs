use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct MarkovModel {
    pub id: i32,
    pub guild_id: String,
    pub current_word: String,
    pub next_word: Option<String>,
    pub frequency: i32,
}

impl MarkovModel {
    pub fn default() -> Self {
        MarkovModel {
            id: -1,
            guild_id: "".to_string(),
            current_word: "".to_string(),
            next_word: None,
            frequency: -1,
        }
    }
}