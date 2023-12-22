create table settings
(
    id       serial primary key,
    guild_id bigint  not null,
    setting  varchar not null,
    val      varchar
);

alter table settings
    add constraint def unique (guild_id, setting);