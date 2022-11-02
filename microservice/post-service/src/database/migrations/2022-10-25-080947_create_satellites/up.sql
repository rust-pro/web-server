create table main.satellites
(
    id                            serial primary key,
    name                          varchar                  not null unique,
    life_exists                   varchar(20)              not null,
    first_spacecraft_landing_date date,
    planet_id                     integer                  not null,
    created_at                    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at                    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
