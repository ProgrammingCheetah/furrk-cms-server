-- Add migration script here
CREATE TABLE e621_tag (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE e621_default_tag (
    id SERIAL PRIMARY KEY,
    e621_tag_id INT NOT NULL,
    FOREIGN KEY (e621_tag_id) REFERENCES e621_tag (id)
);

CREATE TABLE e621_forbidden_tag (
    id SERIAL PRIMARY KEY,
    e621_tag_id INT NOT NULL,
    FOREIGN KEY (e621_tag_id) REFERENCES e621_tag (id)
);