-- Your SQL goes here
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);