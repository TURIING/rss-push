-- Your SQL goes here
create table subscribe (
    id integer primary key autoincrement,
    username varchar not null,
    crate_id varchar not null,
    CONSTRAINT fk_crates  
    FOREIGN KEY (crate_id)  
    REFERENCES crates(crate_id)


)