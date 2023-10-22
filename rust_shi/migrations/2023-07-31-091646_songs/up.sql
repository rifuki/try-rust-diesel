-- Your SQL goes here
CREATE TABLE songs (
    id UUID PRIMARY KEY NOT NULL,
    anthropomorphic_id UUID NOT NULL,
    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    release_year INTEGER NOT NULL,
    FOREIGN KEY (anthropomorphic_id) REFERENCES anthropomorphism(id)
);