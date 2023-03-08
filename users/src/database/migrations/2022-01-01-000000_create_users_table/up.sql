create table main.users
(
    id         serial primary key,
    username   varchar(50)  not null unique,
    email      varchar(255) not null unique,
    role       varchar(500) not null,
    password   varchar(500) not null,
    created_at timestamptz  not null default NOW(),
    updated_at timestamptz  not null default NOW(),
    deleted_at timestamptz
);
