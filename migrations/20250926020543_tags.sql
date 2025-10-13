CREATE TYPE tag_type AS ENUM ('default', 'forbidden');

-- Add migration script here
CREATE TABLE e621_tag (
    id SERIAL PRIMARY KEY,
    name VARCHAR(512) NOT NULL,
    type tag_type NOT NULL
);