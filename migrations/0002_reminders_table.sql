create table reminders
(
    id         serial primary key,
    channel_id varchar   not null,
    user_id    varchar   not null,
    message    varchar(2000),
    remind_at  timestamp with time zone not null,
    state      state not null
);