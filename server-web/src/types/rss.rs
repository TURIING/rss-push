use rocket::serde::{Deserialize, Serialize, de::value::StrDeserializer};
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
    crates_id: String,
    crates_type: String,
    info: String
}


#[derive(Insertable, AsChangeset)]
#[table_name = "task"]
pub struct TaskQuery {
    crates_id: String,
    task_type: String,
    username: String,
    params: String

}