-- Your SQL goes here
CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR unique NOT NULL,
    passwd VARCHAR NOT NULL
)