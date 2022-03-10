-- Your SQL goes here
create table task(
    crates_id varchar primary key not null,
    task_type varchar not null,
    username varchar not null,
    params varchar not null
)