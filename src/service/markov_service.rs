use sqlx::{Postgres, Pool, Error, query, Row, query_as};
use rand::{Rng, thread_rng};
use crate::models::MarkovModel;

pub struct MarkovService;

impl MarkovService {
    pub async fn store(db: &Pool<Postgres>, part: MarkovModel) {
        if let Some(id) = MarkovService::check_if_exists(db, &part).await {
            if let Some(freq) = MarkovService::get_frequency(db, id).await {
                let _ = MarkovService::update(db, id, freq + 1).await;
            }
        } else {
            let _ = MarkovService::insert(db, &part).await;
        }
    }

    async fn insert(db: &Pool<Postgres>, part: &MarkovModel) -> Result<i32, Error> {
        debug!("inserting words '{}' - '{:?}'", part.current_word, part.next_word);

        match query("insert into markov_data \
        (guild_id, current_word, next_word, frequency)\
         values($1,$2,$3,$4) returning id")
            .bind(&part.guild_id)
            .bind(&part.current_word)
            .bind(&part.next_word)
            .bind(&part.frequency)
            .fetch_one(db).await {
            Ok(row) => Ok(row.get::<i32, _>(0)),
            Err(e) => Err(e)
        }
    }

    async fn update(db: &Pool<Postgres>, id: i32, freq: i32) -> Result<(), Error> {
        debug!("updating id {} - new freq {}", id, freq);

        match query("update markov_data set frequency = $1 where id = $2")
            .bind(freq)
            .bind(id)
            .fetch_one(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    async fn check_if_exists(db: &Pool<Postgres>, part: &MarkovModel) -> Option<i32> {
        match query("select id from markov_data where current_word = $1 and next_word = $2 and guild_id = $3")
            .bind(&part.current_word)
            .bind(&part.next_word)
            .bind(&part.guild_id)
            .fetch_optional(db).await {
            Ok(d) => {
                match d {
                    Some(row) => Some(row.get::<i32, _>(0)),
                    None => None
                }
            }
            Err(e) => {
                warn!("check_if_exists: {}", e);
                None
            }
        }
    }

    async fn get_frequency(db: &Pool<Postgres>, id: i32) -> Option<i32> {
        match query("select frequency from markov_data where id = $1")
            .bind(id)
            .fetch_optional(db).await {
            Ok(d) => {
                match d {
                    Some(row) => Some(row.get::<i32, _>(0)),
                    None => None
                }
            }
            Err(e) => {
                warn!("get_frequency: {}", e);
                None
            }
        }
    }

    async fn get_max(db: &Pool<Postgres>) -> i32 {
        match query("select max(id) from markov_data")
            .fetch_one(db).await {
            Ok(row) => row.get::<i32, _>(0),
            Err(e) => {
                warn!("get_max: {}", e);
                0
            }
        }
    }

    async fn get_by_id(db: &Pool<Postgres>, id: i32) -> Result<MarkovModel, Error> {
        query_as("select * from markov_data where id = $1")
            .bind(id)
            .fetch_one(db).await
    }

    pub async fn generate_message(db: &Pool<Postgres>) -> String {
        let max = MarkovService::get_max(db).await;
        debug!("generating message with {} entries", max);
        let mut msg = String::new();

        if max < 1000 {
            warn!("there is not enough data to work with");
            return msg;
        }

        let start_id = thread_rng().gen_range(1..=max);

        if let Ok(start) = MarkovService::get_by_id(db, start_id).await {
            let mut part = start;

            while msg.len() < 2000 {
                msg.push_str(format!("{} ", part.current_word).as_str());

                if let Some(next) = part.next_word {
                    part = MarkovService::get_next(db, next, part.guild_id).await;
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

    async fn get_next(db: &Pool<Postgres>, current_word: String, from_guild: String) -> MarkovModel {
        if let Ok(possibilities) =
            query_as::<_, MarkovModel>("select * from markov_data where current_word = $1 and guild_id = $2")
                .bind(current_word)
                .bind(from_guild)
                .fetch_all(db).await {
            let idx = thread_rng().gen_range(0..possibilities.len());
            return possibilities[idx].clone()
        } else {
            error!("cannot fetch next markov part");
            MarkovModel::empty()
        }
    }
}