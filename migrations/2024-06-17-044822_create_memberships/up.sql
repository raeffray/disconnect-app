CREATE TABLE memberships (
    id SERIAL PRIMARY KEY,
    code VARCHAR NOT NULL,
    status_in_platform VARCHAR NOT NULL
);