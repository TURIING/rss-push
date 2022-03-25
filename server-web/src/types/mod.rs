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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[non_exhaustive]
pub struct SuccessStatus;
impl SuccessStatus {
    pub const REGISTER: i32 = 200;
    pub const LOGIN: i32 = 201;
    pub const SUBSCRIBE: i32 = 202;
    pub const RSSINFO:i32 = 203;
}