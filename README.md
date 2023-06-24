# murkov

markov chain discord bot in rust + serenity + postgres

example `.env` file in project root directory:

```
POSTGRES_USER=
POSTGRES_PASSWORD=
HOST=
PORT=
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${HOST}:${PORT}/${POSTGRES_USER}
BOT_TOKEN=
RUST_LOG=murkov=debug
```