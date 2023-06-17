use std::error::Error;
use std::env::var;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let url = var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(())
}

// https://users.rust-lang.org/t/how-to-run-a-function-after-a-time-delay/86260
// https://invidious.nerdvpn.de/watch?v=TCERYbgvbq0