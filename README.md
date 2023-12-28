# slykbots

I bring back old features from bots that are now offline


## features

- [x] markov-chain bot (murkov)
- [x] ai features (simple open ai api example)
- [x] basically rythm bot (vc youtube/music bot) (also murkov)
- [ ] soundcloud support
- [x] reddit polling
- [ ] fix broken reddit links
- [ ] split music from murkov into new
- [x] fix rust workspace shit - it's separated packages now
- [ ] clean up code
- [ ] ~~not quite nitro as second bot (fnb soon)~~

___

`.env` file so I don't get dementia

```
POSTGRES_USER=
OPENAI_API_KEY=
MAX_USER_PROMPTS=
POSTGRES_PASSWORD=
HOST=
PORT=
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${HOST}:${PORT}/${POSTGRES_USER}
MURKOV_TOKEN=
RPP_TOKEN=
RUST_LOG=murkov=debug,rpp=debug
AI=1
MIN_WORDS=3
OPENAI_KEY=
MAX_USER_PROMPTS=
AI_MODEL_PROMPTS=gpt-3.5-instruct
AI_MODEL_CHAT=gpt-3.5-turbo
MAX_TOKENS=300
MAX_ASK_PER_USER_PER10MIN=10
MAX_PING_PER_USER_PER10MIN=1
PROMPT_BASE=
FEED_PAUSE=
PAGE=
TOGGLE_DISABLED
```

