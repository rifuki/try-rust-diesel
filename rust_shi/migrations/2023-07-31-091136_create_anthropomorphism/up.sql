-- Your SQL goes here
CREATE TYPE hair_color_enum AS ENUM ('turquoise', 'pink', 'blue', 'yellow', 'green');
CREATE TYPE gender_enum AS ENUM ('male', 'female');
CREATE TABLE anthropomorphism (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    age INTEGER NOT NULL,
    hair_color hair_color_enum NOT NULL,
    gender gender_enum NOT NULL
);
