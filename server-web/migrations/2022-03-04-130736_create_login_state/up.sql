-- Your SQL goes here
create table login_state (
    id integer primary key autoincrement,
    username varchar unique not null,
    session_data varchar not null
)