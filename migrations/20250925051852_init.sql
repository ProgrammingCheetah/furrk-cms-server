-- Add migration script here
CREATE TABLE channel_user (
    id SERIAL PRIMARY KEY,
    telegram_id VARCHAR(20) NOT NULL
);

CREATE TABLE file_type (
    id SERIAL PRIMARY KEY,
    f_type VARCHAR(10) UNIQUE NOT NULL
);

CREATE TABLE media_item (
    id SERIAL PRIMARY KEY,
    platform VARCHAR(255) NOT NULL,
    source_url VARCHAR(512) UNIQUE NOT NULL,
    media_url TEXT UNIQUE NOT NULL,
    author_id INT,
    file_type_id INT,
    FOREIGN KEY (author_id) REFERENCES channel_user (id),
    FOREIGN KEY (file_type_id) REFERENCES file_type (id)
);