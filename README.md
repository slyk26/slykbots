# murkov

markov chain discord bot with serenity + postgres 

example `.env` file in project root directory:

```
POSTGRES_USER=
OPENAI_API_KEY=
MAX_USER_PROMPTS=
POSTGRES_PASSWORD=
HOST=
PORT=
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${HOST}:${PORT}/${POSTGRES_USER}
BOT_TOKEN=
RUST_LOG=murkov=debug
```

### TODO

- [x] add openai api in dms and `ask` legacy command
- [ ] optimize and debug "chat" mode of AI model
- [ ] .env file variable for loaded modules so i don't go broke
- [ ] refactor existing code
- [ ] fixing markov message generation (don't use equal chance)
- [ ] add modes for message generation (per server/user?)


#### possible things in the future

- [ ] making + hosting own ai model