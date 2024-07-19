-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL UNIQUE,
    secret VARCHAR NOT NULL
);