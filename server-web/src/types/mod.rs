pub mod auth;
pub mod schema;

use rocket::serde::{Serialize, Deserialize};
use crate::rss::RssInfo;


// the response message body
#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ResMsg {
    pub status: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
    #[serde(rename(serialize = "rssInfo"))]
    #[serde(skip_serializing_if = "RssInfo::is_empty")]
    pub rss_info: RssInfo,
    #[serde(rename(serialize = "cratesInfo"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub crates_info: Vec<ResCrateInfo>,
    #[serde(rename(serialize = "messagesInfo"))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages_info: Vec<ResMessagesInfo>

}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMessagesInfo {
    pub title: String,
    pub description: String,
    pub content: String,
    pub message_id: String,
    pub crate_id: String,
    pub check_status: bool,
    pub send_time: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResCrateInfo {
    #[serde(rename(serialize = "crateId"))]
    pub crate_id: String,
    #[serde(rename(serialize = "crateInfo"))]
    pub crate_info: CrateInfo
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

#[non_exhaustive]
pub struct SuccessStatus;
impl SuccessStatus {
    pub const REGISTER: i32 = 200;
    pub const LOGIN: i32 = 201;
    pub const SUBSCRIBE: i32 = 202;
    pub const RSSINFO:i32 = 203;
    pub const SUBSCRIBED_SOME: i32 = 204;
    pub const SUBSCRIBED_NONE: i32 = 205;
    pub const REFRESHMESSAGE_SOME: i32 = 206;
    pub const REFRESHMESSAGE_NONE: i32 = 209;
    pub const READMESSAGE: i32 = 207;
    pub const UNREADMESSAGE: i32 = 208;
    pub const UNSUBSCRIBE: i32 = 210;
}