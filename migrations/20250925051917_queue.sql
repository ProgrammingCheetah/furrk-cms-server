-- Add migration script here
CREATE TABLE queue (
    id SERIAL PRIMARY KEY,
    media_item_id INT UNIQUE,
    FOREIGN KEY (media_item_id) REFERENCES media_item (id)
);