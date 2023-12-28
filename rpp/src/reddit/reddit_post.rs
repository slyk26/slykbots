use std::hash::{Hash, Hasher};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Clone, Ord, PartialOrd, Eq)]
pub struct RedditPost {
    pub id: String,
    pub poster: String,
    pub title: String,
    pub src: String,
    pub uploaded: DateTime<Utc>,
    pub t: PostType,
    pub subreddit: String,
}

impl PartialEq for RedditPost {
    fn eq(&self, other: &Self) -> bool {
        // literally same post
        self.id.eq(&other.id) ||
            // same spamposting retard
            (self.title.eq(&other.title) && self.poster.eq(&other.poster))
    }
}


impl Hash for RedditPost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.poster.hash(state);
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PostType {
    Text,
    Media,
    Gallery,
    Video,
    External,
    DiscordInvite
}