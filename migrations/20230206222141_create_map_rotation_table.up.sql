-- Add up migration script here
CREATE TABLE map_rotations (
    id SMALLINT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    code TINYTEXT NOT NULL
);