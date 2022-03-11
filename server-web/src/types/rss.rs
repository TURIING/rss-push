use rocket::serde::{Deserialize, Serialize};
use crate::types::database::{ task, crates };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RssInfo {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SubscribeInfo {
    pub url: String,
    pub session_data: String
}


#[derive(Insertable, AsChangeset)]
#[table_name = "crates"]
pub struct CratesQuery {
    pub crates_id: String,
    pub crates_type: String,
    pub info: String
}


#[derive(Insertable, AsChangeset)]
#[table_name = "task"]
pub struct TaskQuery {
    pub crates_id: String,
    pub task_type: String,
    pub username: String,
    pub params: String

}