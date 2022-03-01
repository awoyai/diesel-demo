create table posts
(
    id   bigint auto_increment not null,
    name varchar(256) null,
    constraint posts_pk
        primary key (id)
);

create unique index posts_id_uindex
    on posts (id);