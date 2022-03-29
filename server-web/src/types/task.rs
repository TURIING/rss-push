use diesel::Queryable;
use rocket::serde::{ Deserialize, Serialize};

use crate::types::database::{ task, crates, subscribe };

#[derive(Insertable, AsChangeset)]
#[table_name = "crates"]
pub struct CratesQuery {
    pub crate_id: String,
    pub crate_type: String,
    pub info: String
}

#[derive(Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CrateInfo {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String
}


#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "task"]
pub struct TaskQuery {
    pub id: Option<i32>,
    pub crate_id: String,
    pub task_type: String,
    pub username: String,
    pub params: String

}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "subscribe"]
pub struct SubscribeQuery {
    pub id: Option<i32>,
    pub username: String,
    pub crate_id: String
}