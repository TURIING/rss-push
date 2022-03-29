-- Your SQL goes here
create table message_content (
    id integer primary key,
    message_id varchar not null,
    send_time varchar not null,
    content text not null

)