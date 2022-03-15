-- Your SQL goes here
create table login_state (
    id integer primary key autoincrement,
    username varchar not null,
    token varchar not null
)