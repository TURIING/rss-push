pub mod auth;
pub mod database;

use rocket::{
    response::status,
    serde::{json::Value, Serialize},
};
// the response message body
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMsg {
    pub status: i32,
    pub msg: String,
}
//  the response msg type
pub type ResType = status::Custom<Value>;
