-- Your SQL goes here
create table message (
    id integer primary key,
    message_id varchar not null,
    crate_id varchar not null,
    receiver varchar not null,
    check_status varchar not null
)