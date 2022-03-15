use rocket::serde::{Deserialize, Serialize};

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


