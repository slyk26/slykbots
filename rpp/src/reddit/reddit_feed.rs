use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use feed_rs::model::{Entry, Feed, Person};
use feed_rs::parser;
use feed_rs::parser::ParseFeedError;
use log::{debug, error};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time;
use shared::serenity_utils::say;
use crate::reddit::{PostType, RedditPost};
use crate::reddit::local_cache::{add_to_cache, add_to_queue, get_queue_item};
use crate::subscriptions::SubService;

pub async fn fetch_thread() {
    let feed_pause = env::var("FEED_PAUSE").unwrap_or("60".to_string()).parse::<u64>().unwrap_or(60);
    let mut i = time::interval(Duration::from_secs(feed_pause));
    let scm = map_subscriptions().await;

    tokio::task::spawn(async move {
        loop {
            debug!("waiting for {feed_pause}s");
            i.tick().await;
            let mut js = JoinSet::new();

            for a in &scm {
                a.1.iter().for_each(|_| {
                    js.spawn(create_feed_handle(a.0.clone()));
                });
            }

            while let Some(res) = js.join_next().await {
                if let Err(e) = res {
                    error!("{e}");
                }
            }
        }
    });
}

pub async fn post_thread(http: Arc<Http>) {
    let mut i = time::interval(Duration::from_secs(1));
    let scm = map_subscriptions().await;
    tokio::task::spawn(async move {
        loop {
            i.tick().await;
            if let Some(post) = get_queue_item().await {
                debug!("peng!");
                let channel_ids: &Vec<u64> = scm.get(&post.subreddit).unwrap();

                for id in channel_ids {
                    say(ChannelId::from(*id), &http, post.src.clone()).await;
                }
            }
        }
    });
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
    let page = env::var("PAGE").unwrap_or("new".to_string());
    debug!("looking for bullets in {subreddit}");
    let mut content = get_request(format!("https://old.reddit.com/r/{subreddit}/{page}/.rss")).await;
    content = String::from(&content.as_str()[content.find('>').unwrap_or(0) + 1..]);
    parser::parse(content.as_bytes())
}

async fn get_request(url: String) -> String {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36").build().unwrap();
    client.get(url).send().await.unwrap().text().await.unwrap()
}

fn parse_entries(e: Vec<Entry>, sub: String) -> Vec<RedditPost> {
    let st = "<span><a href=\"";
    let end = "\">[link]";

    e.iter().map(|en| {
        let c = en.content.as_ref().unwrap().body.as_ref().unwrap().as_str();
        let link = String::from(&c[c.find(st).unwrap() + st.len()..c.find(end).unwrap()]);
        let typ = determine_posttype(&link);
        RedditPost {
            id: en.id.to_string(),
            poster: en.authors.get(0).unwrap_or(&Person {
                name: "dummy".to_string(),
                uri: None,
                email: None,
            }).clone().name,
            title: en.title.clone().unwrap().content,
            src: link,
            uploaded: en.published.unwrap(),
            t: typ,
            subreddit: sub.clone(),
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

    if link.contains("v.redd.it") {
        return PostType::Video;
    }

    if link.contains("i.redd.it") {
        return PostType::Media;
    }

    if link.contains("discord.gg") {
        return PostType::DiscordInvite;
    }

    PostType::External
}

async fn create_feed_handle(subreddit: String) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        match fetch_feed(subreddit.as_str()).await {
            Ok(feed) => {
                let posts: Vec<_> = parse_entries(feed.entries, subreddit);
                let media_posts: Vec<_> = posts.iter().filter(|p| PostType::Media.eq(&p.t) || PostType::External.eq(&p.t)).cloned().collect();

                for post in media_posts {
                    if add_to_cache(post.clone()).await {
                        debug!("+1 bullet");
                        add_to_queue(post).await;
                    }
                }
            }
            Err(e) => {
                // ratelimited? banned?
                error!("{e}");
            }
        }
    })
}