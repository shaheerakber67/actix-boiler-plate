-- Your SQL goes here
-- Create the field_sector table
CREATE TABLE room (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL 
);

-- Create the field_sector_translation table
CREATE TABLE meeting (
    id SERIAL PRIMARY KEY,
    room_id INT NOT NULL REFERENCES room (id) ON DELETE CASCADE,
    lang_code VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);
