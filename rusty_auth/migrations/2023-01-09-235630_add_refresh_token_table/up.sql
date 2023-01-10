-- Your SQL goes here
CREATE TABLE refresh_tokens(
    id          UUID            PRIMARY KEY     NOT NULL    UNIQUE,
    user_id     UUID            NOT NULL        REFERENCES users(id),
    token       VARCHAR(150)    NOT NULL        UNIQUE,
    valid_until TIMESTAMP       NOT NULL
);