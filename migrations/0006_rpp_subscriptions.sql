create table rpp_subscriptions
(
    id         serial primary key,
    channel_id bigint  not null,
    subreddit  varchar not null
);