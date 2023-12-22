use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RedditPost {
    pub id: String,
    pub poster: String,
    pub title: String,
    pub src: String,
    pub uploaded: DateTime<Utc>,
    pub t: PostType,
}

impl PartialEq for RedditPost {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PostType {
    Text,
    Media,
    Gallery,
}