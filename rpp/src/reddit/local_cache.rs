use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};
use log::info;
use crate::reddit::RedditPost;

static CACHE: OnceLock<Arc<Mutex<HashSet<RedditPost>>>> = OnceLock::new();

static QUEUE: OnceLock<Arc<Mutex<Vec<RedditPost>>>> = OnceLock::new();

pub async fn add_to_cache(r: RedditPost) -> bool {
    let a = CACHE.get_or_init(|| Arc::new(Mutex::new(HashSet::new())));
    let mut b = a.lock().unwrap();
    let c = b.deref_mut();

    if c.len() > 250 {
        info!("cleaning cache");
        let _ = CACHE.set(Arc::new(Mutex::new(HashSet::new())));
        return false;
    }

    c.insert(r)
}

pub async fn add_to_queue(r: RedditPost) {
    let a = QUEUE.get_or_init(|| Arc::new(Mutex::new(vec![])));
    let mut b = a.lock().unwrap();
    let c = b.deref_mut();

    c.insert(0, r);
}

pub async fn get_queue_item() -> Option<RedditPost> {
    let a = QUEUE.get_or_init(|| Arc::new(Mutex::new(vec![])));
    let mut b = a.lock().unwrap();
    let c = b.deref_mut();

    c.pop()
}
