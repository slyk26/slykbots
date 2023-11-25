use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct AiCacheModel {
    pub id: i32,
    pub channel_id: i64,
    pub prompt: String
}