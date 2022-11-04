create table main.user_profiles
(
    id         serial primary key,
    user_id    integer references main.users not null,
    first_name varchar(255),
    last_name  varchar(255),
    image      varchar(1000),
    sex        varchar(255),
    birthday   date,
    created_at timestamptz              not null default NOW(),
    updated_at timestamptz              not null default NOW(),
    deleted_at timestamptz
);
