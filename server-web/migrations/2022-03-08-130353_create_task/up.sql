-- Your SQL goes here
create table task(
    id integer primary key autoincrement,
    crate_id varchar not null,
    task_type integer not null,
    username varchar not null,
    params varchar
)