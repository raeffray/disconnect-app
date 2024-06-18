CREATE TABLE fellows (
    id SERIAL PRIMARY KEY,
    membership_id INTEGER NOT NULL REFERENCES memberships(id),
    fellowship_type VARCHAR NOT NULL
);