CREATE TABLE members (
    id SERIAL PRIMARY KEY,
    membership_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    FOREIGN KEY (membership_id) REFERENCES memberships (id)
);