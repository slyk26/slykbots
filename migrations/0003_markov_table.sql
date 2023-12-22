create table markov_data
(
    id           serial primary key,
    guild_id     varchar not null,
    current_word varchar not null,
    next_word    varchar,
    frequency    integer not null
)