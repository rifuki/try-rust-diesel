-- Your SQL goes here
CREATE TYPE role_enum AS ENUM ('admin', 'user');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    role role_enum NOT NULL DEFAULT 'user'
);