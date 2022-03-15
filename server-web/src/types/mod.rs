pub mod auth;
pub mod database;
pub mod rss;
pub mod task;

use rocket::serde::Serialize;

// the response message body
#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ResMsg {
    pub status: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
}
