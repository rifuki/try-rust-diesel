-- Your SQL goes here
CREATE TYPE users_role_enum AS ENUM ('superadmin', 'admin', 'user');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    role users_role_enum NOT NULL DEFAULT 'user'
);
