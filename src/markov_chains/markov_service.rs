use std::env;
use sqlx::{Error, query, Row, query_as};
use rand::{Rng, thread_rng};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::markov_chains::markov_model::MarkovModel;
use crate::PG;

pub struct MarkovService;

impl MarkovService {
    async fn store(part: MarkovModel) {
        if let Some(id) = MarkovService::check_if_exists(&part).await {
            if let Some(freq) = MarkovService::get_frequency(id).await {
                let _ = MarkovService::update(id, freq + 1).await;
            }
        } else {
            let _ = MarkovService::insert(&part).await;
        }
    }

    async fn insert(part: &MarkovModel) -> Result<i32, Error> {
        debug!("inserting words '{}' - '{:?}'", part.current_word, part.next_word);

        match query("insert into markov_data \
        (guild_id, current_word, next_word, frequency)\
         values($1,$2,$3,$4) returning id")
            .bind(&part.guild_id)
            .bind(&part.current_word)
            .bind(&part.next_word)
            .bind(part.frequency)
            .fetch_one(PG.get().unwrap()).await {
            Ok(row) => Ok(row.get::<i32, _>(0)),
            Err(e) => Err(e)
        }
    }

    async fn update(id: i32, freq: i32) -> Result<(), Error> {
        debug!("updating id {} - new freq {}", id, freq);

        match query("update markov_data set frequency = $1 where id = $2")
            .bind(freq)
            .bind(id)
            .fetch_one(PG.get().unwrap()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    async fn check_if_exists(part: &MarkovModel) -> Option<i32> {
        match query("select id from markov_data where current_word = $1 and next_word = $2 and guild_id = $3")
            .bind(&part.current_word)
            .bind(&part.next_word)
            .bind(&part.guild_id)
            .fetch_optional(PG.get().unwrap()).await {
            Ok(d) => {
                d.map(|row| row.get::<i32, _>(0))
            }
            Err(e) => {
                warn!("check_if_exists: {}", e);
                None
            }
        }
    }

    async fn get_frequency(id: i32) -> Option<i32> {
        match query("select frequency from markov_data where id = $1")
            .bind(id)
            .fetch_optional(PG.get().unwrap()).await {
            Ok(d) => {
                d.map(|row| row.get::<i32, _>(0))
            }
            Err(e) => {
                warn!("get_frequency: {}", e);
                None
            }
        }
    }

    pub async fn get_max(guild_id: &String) -> i64 {
        match query("select count(*) from markov_data where guild_id = $1")
            .bind(guild_id)
            .fetch_one(PG.get().unwrap()).await {
            Ok(row) => row.get::<i64, _>(0),
            Err(e) => {
                warn!("get_max: {}", e);
                0
            }
        }
    }

    async fn get_start_model(guild_id: &String) -> Result<MarkovModel, Error> {
        query_as("select * from markov_data where guild_id = $1 order by random() limit 1")
            .bind(guild_id)
            .fetch_one(PG.get().unwrap()).await
    }

    async fn generate_message(guild_id: String) -> String {
        let max = MarkovService::get_max(&guild_id).await;
        debug!("generating message with {} entries", max);
        let mut msg = String::new();

        if max < 1000 {
            warn!("[{}] not enough markov entries: {}",guild_id, max);
            return msg;
        }

        if let Ok(start) = MarkovService::get_start_model(&guild_id).await {
            let mut part = start;
            let mut current_word = 0;

            while msg.len() < 2000 {
                msg.push_str(format!("{} ", part.current_word).as_str());
                current_word += 1;

                if let Some(next) = part.next_word {
                    part = MarkovService::get_next(next, part.guild_id).await;
                } else if env::var("MIN_WORDS").unwrap_or(String::from("9999")).parse::<i32>().unwrap() > current_word {
                    part = MarkovService::get_start_model(&guild_id).await.unwrap();
                } else {
                    debug!("return message: {}", msg);
                    return msg;
                }
            }
        } else {
            // cannot get part with id
            warn!("cannot start message generation")
        }
        msg
    }

    async fn get_next(current_word: String, from_guild: String) -> MarkovModel {
        if let Ok(possibilities) =
            query_as::<_, MarkovModel>("select * from markov_data where current_word = $1 and guild_id = $2 order by frequency desc")
                .bind(&current_word)
                .bind(&from_guild)
                .fetch_all(PG.get().unwrap()).await {

            let freqs: Vec<i32> = possibilities.iter().map(| p | p.frequency).collect();
            let randomshit = thread_rng().gen_range(0..freqs.iter().sum());
            MarkovService::find_element(&possibilities, randomshit).unwrap_or(MarkovModel::default())
        } else {
            error!("cannot fetch next markov part");
            MarkovModel::default()
        }
    }

    fn find_element(vec: &[MarkovModel], f: i32) -> Option<MarkovModel> {
        let mut sum = 0;

        for (i, element) in vec.iter().enumerate() {
            if i != 0 {
                sum += vec[i - 1].frequency;
            }
            if sum + element.frequency > f {
                return Some(element.clone());
            }
        }
        None
    }

    pub async fn send_message(ctx: &Context, msg: &Message) {
        if !msg.author.bot {
            let message = MarkovService::generate_message(msg.guild_id.unwrap().to_string()).await;
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(message)
            }).await;
        }
    }

    pub async fn destruct_message(msg: &Message) {
        if !msg.author.bot {
            let words = msg.content.split(' ').collect::<Vec<&str>>();

            for i in 0..words.len() {
                let current = words[i].to_string();
                let mut next: Option<String> = None;
                if i + 1 < words.len() {
                    next = Some(words[i + 1].to_string());
                }
                MarkovService::store(MarkovModel {
                    id: 0,
                    guild_id: msg.guild_id.unwrap().to_string(),
                    current_word: current,
                    next_word: next,
                    frequency: 1,
                }).await;
            }
        }
    }

    pub async fn get_stats(guild_id: &String) -> (i64, i64) {
        let db = PG.get().unwrap();
        let mut entries = -1;
        let mut used = -1;

        match query("select count(*) from markov_data where guild_id = $1")
            .bind(guild_id)
            .fetch_one(db).await {
            Ok(row) => entries = row.get::<i64, _>(0),
            Err(e) => warn!("cannot get entries: {}", e)
        }

        match query("select count(distinct guild_id) from markov_data")
            .fetch_one(db).await {
            Ok(row) => used = row.get::<i64, _>(0),
            Err(e) => warn!("cannot get used servers: {}", e)
        }

        (entries, used)
    }
}