-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL NOT NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);