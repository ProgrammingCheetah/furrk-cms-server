-- Add migration script here
CREATE TYPE platform AS ENUM ('twitter', 'e621', 'furaffinity');

CREATE TABLE channel_user (
    id SERIAL PRIMARY KEY,
    telegram_id VARCHAR(20) NOT NULL
);

CREATE TABLE media_item (
    id SERIAL PRIMARY KEY,
    platform platform NOT NULL,
    source_url TEXT UNIQUE NOT NULL,
    media_url TEXT UNIQUE NOT NULL,
    author_id INT,
    FOREIGN KEY (author_id) REFERENCES channel_user (id)
);