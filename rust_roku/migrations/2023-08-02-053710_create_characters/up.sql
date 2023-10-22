-- Your SQL goes here
CREATE TYPE gender_enum AS ENUM ('male', 'female');

CREATE TYPE hair_color_enum AS ENUM ('turquoise', 'yellow', 'pink');

CREATE TABLE characters (
    id UUID PRIMARY KEY NOT NULL,
    code_name VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    age INTEGER NOT NULL,
    gender gender_enum NOT NULL,
    hair_color hair_color_enum NOT NULL,
    FOREIGN KEY (code_name) REFERENCES code_names(id)
);