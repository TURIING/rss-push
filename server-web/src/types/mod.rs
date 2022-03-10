pub mod auth;
pub mod database;
pub mod rss;

use rocket::{
    response::status,
    serde::{json::Value, Serialize},
};
// the response message body
#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ResMsg {
    pub status: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub session_data: String,
}
//  the response msg type
pub type ResType = status::Custom<Value>;
