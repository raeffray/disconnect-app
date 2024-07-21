-- Your SQL goes here
CREATE TABLE systemusers (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL UNIQUE,
    secret VARCHAR NOT NULL
);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE system_user_roles (
    system_user_id INT NOT NULL,
    role_id INT NOT NULL,
    PRIMARY KEY (system_user_id, role_id),
    FOREIGN KEY (system_user_id) REFERENCES systemusers(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

