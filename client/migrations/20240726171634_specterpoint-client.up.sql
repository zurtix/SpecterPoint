-- Add up migration script here
CREATE TABLE user (
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    data TEXT NOT NULL
);
