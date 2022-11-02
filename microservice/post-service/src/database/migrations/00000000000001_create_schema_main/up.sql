CREATE SCHEMA IF NOT EXISTS main
    AUTHORIZATION postgres;

COMMENT
    ON SCHEMA main
    IS 'standard main schema';

GRANT ALL
    ON SCHEMA main TO PUBLIC;

GRANT ALL
    ON SCHEMA main TO postgres;
