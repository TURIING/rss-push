use rocket::{
    form::{ Form, Strict},
    serde::json::{Value, serde_json::json},
};
use rss::Channel;


#[post("/info", data = "<url>")]
pub async fn rss_info(url: Form<Strict<String>>) -> Value {
    
    let content = reqwest::get(url.as_str()).await.unwrap().bytes().await.unwrap();
    let channel = Channel::read_from(&content[..]).unwrap();
    json!(channel.description())
}