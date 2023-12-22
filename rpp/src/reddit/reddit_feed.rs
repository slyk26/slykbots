use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use feed_rs::model::{Entry, Feed};
use feed_rs::parser;
use feed_rs::parser::ParseFeedError;
use log::{debug, error, warn};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use tokio::sync::Mutex;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time;
use shared::serenity_utils::say;
use crate::reddit::{PostType, RedditPost};
use crate::subscriptions::SubService;

pub async fn subreddit_threads(cache: Arc<Mutex<HashMap<String, Vec<RedditPost>>>>, http: Arc<Http>) {
    let mut i = time::interval(Duration::from_secs(60));
    let scm = map_subscriptions().await;

    loop {
        i.tick().await;
        let mut js = JoinSet::new();

        for a in &scm {
            a.1.iter().for_each(|channel_id| {
                js.spawn(create_feed_handle(a.0.clone(), *channel_id, cache.clone(), http.clone()));
            });
        }

        while let Some(res) = js.join_next().await {
            if let Err(e) = res {
                error!("{e}");
            }
        }
    }
}

async fn map_subscriptions() -> HashMap<String, Vec<u64>> {
    let mut scm: HashMap<String, Vec<u64>> = HashMap::new();
    SubService::get_all().await.iter().for_each(|sub| {
        let mut a = match scm.get(&sub.subreddit).cloned() {
            Some(d) => d,
            None => vec![]
        };
        a.push(sub.channel_id as u64);
        scm.insert(sub.subreddit.clone(), a);
    });
    scm
}

async fn fetch_feed(subreddit: &str) -> Result<Feed, ParseFeedError> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36").build().unwrap();
    let mut content = client.get(format!("https://old.reddit.com/r/{subreddit}/new/.rss")).send()
        .await.unwrap().text().await.unwrap();

    content = String::from(&content.as_str()[content.find('>').unwrap_or(0) + 1..]);

    parser::parse(content.as_bytes())
}

fn parse_entries(e: Vec<Entry>) -> Vec<RedditPost> {
    let st = "<span><a href=\"";
    let end = "\">[link]";

    e.iter().map(|en| {
        let c = en.content.as_ref().unwrap().body.as_ref().unwrap().as_str();
        let link = String::from(&c[c.find(st).unwrap() + st.len()..c.find(end).unwrap()]);
        let typ = determine_posttype(&link);
        RedditPost {
            id: en.id.to_string(),
            poster: en.authors.get(0).unwrap().clone().name,
            title: en.title.clone().unwrap().content,
            src: link,
            uploaded: en.published.unwrap(),
            t: typ,
        }
    }).collect::<Vec<RedditPost>>()
}

fn determine_posttype(link: &str) -> PostType {
    if link.contains("/gallery") {
        return PostType::Gallery;
    }

    if link.contains("/comments/") {
        return PostType::Text;
    }

    PostType::Media
}

async fn create_feed_handle(subreddit: String, channel_id: u64, cache: Arc<Mutex<HashMap<String, Vec<RedditPost>>>>, http: Arc<Http>) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        match fetch_feed(subreddit.as_str()).await {
            Ok(feed) => {
                let posts: Vec<_> = parse_entries(feed.entries);
                let mut b: Vec<_> = posts.iter().filter(|p| PostType::Media.eq(&p.t)).cloned().collect();

                b.sort_by(|p, p2| p.uploaded.cmp(&p2.uploaded));

                let mut c = cache.lock().await;
                if c.contains_key(subreddit.as_str()) {
                    let d = c.get(subreddit.as_str()).unwrap();

                    let e: Vec<&RedditPost> = b.iter().filter(|f| !d.contains(f)).collect();

                    debug!("{subreddit} new: {:?}", e);

                    for ie in e {
                        say(ChannelId::from(channel_id), &http, ie.src.clone()).await
                    }
                }
                c.insert(subreddit, b);
            },
            Err(e) => {
                // ratelimited? banned?
                error!("{e}");
            }
        }
    })
}