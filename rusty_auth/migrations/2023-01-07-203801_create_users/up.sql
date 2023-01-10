-- Your SQL goes here
CREATE TABLE users (
    id              UUID            PRIMARY KEY         NOT NULL UNIQUE,
    username        VARCHAR(50)     NOT NULL UNIQUE,
    password_hash   VARCHAR(150)    NOT NULL,
    created_at      TIMESTAMP       NOT NULL
);