use rocket::serde::{ Deserialize, Serialize};

use crate::types::schema::{ task, crates, subscribe, user, message, message_content };

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct AccountInfo {
    pub username: String,
    pub passwd: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user"]
pub struct UserQuery {
    pub id: Option<i32>,
    pub username: String,
    pub passwd: String,
}

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

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "message"]
pub struct MessageQuery {
    pub id: Option<i32>,
    pub message_id: String,
    pub crate_id: String,
    pub receiver: String,
    pub check_status: String,
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "message_content"]
pub struct MessageContentQuery {
    pub id: Option<i32>,
    pub message_id: String,
    pub send_time: String,
    pub content: String
}