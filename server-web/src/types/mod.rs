pub mod auth;
pub mod database;
pub mod task;

use rocket::serde::Serialize;
use crate::rss::RssInfo;

use self::task::CrateInfo;

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
    pub crates_info: Vec<ResCrateInfo>
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResCrateInfo {
    #[serde(rename(serialize = "crateId"))]
    pub crate_id: String,
    #[serde(rename(serialize = "crateInfo"))]
    pub crate_info: CrateInfo
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
}