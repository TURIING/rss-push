-- Your SQL goes here
create table task(
    id integer primary key autoincrement,
    crates_id varchar not null,
    task_type varchar not null,
    username varchar not null,
    params varchar not null
)