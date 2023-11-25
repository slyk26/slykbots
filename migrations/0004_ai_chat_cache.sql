create table ai_chat_cache
(
    id          serial primary key ,
    channel_id  bigint not null,
    prompt      varchar not null
    
)