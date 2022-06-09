use crate::{
    types::{ResMsg, SuccessStatus, auth::Token, CrateInfo},
    DbConn, error::RssError, rss::RssInfo,
    database::{ crates::Crates, task::Task, subscribe::Subscribe, message::Message}
};
use rocket::{
    serde::json::{serde_json::{self, json}, Value},
    http::RawStr,
};
use uuid::Uuid;

// return rss info
#[post("/info", format="application/x-www-form-urlencoded", data="<url>")]
pub async fn info(_token: Token, url: String) -> Result<Value, RssError> {
    let url_original = RawStr::new(&url).url_decode().unwrap();
    let url: Vec<&str> = url_original.split('=').collect();

    let rss_info = RssInfo::new(url[1].to_string()).await?;
    
    Ok(json!(ResMsg{
        status: SuccessStatus::RSSINFO, 
        rss_info: rss_info, 
        ..Default::default()
    }))
}

// subscribe rss feed
#[post("/subscribe", format="application/x-www-form-urlencoded", data="<url>")]
pub async fn subscribe(token: Token, url: String, conn: DbConn) -> Result<Value, RssError> {

    let url_original = RawStr::new(&url).url_decode().unwrap();
    let url: Vec<&str> = url_original.split('=').collect();
    let url = url[1].to_string();

    let url_uuid = Uuid::new_v3(&Uuid::NAMESPACE_URL, url.as_bytes()).to_string();

    let username = token.0;

    let RssInfo{ url: _, title, description } = RssInfo::new(url.clone()).await?;
    let crate_info = serde_json::to_string(&CrateInfo{ url, title, description }).unwrap();
    
    conn.run(move |con| {
        // check exist.
        Crates { 
            crate_id: url_uuid.clone(), 
            crate_type: 0, 
            info: crate_info
        }.exist_or_insert(&con)?;

        // Join the task queue.
        Task {
            crate_id: String::from(url_uuid.clone()),
            username: username.clone(),
            ..Default::default()
        }.insert(con)?;

        Subscribe { username, crate_id: url_uuid, ..Default::default() }.insert(con)?;

        Ok(json!(
            ResMsg{ 
                status: SuccessStatus::SUBSCRIBE, 
                msg: String::from("success"), 
                ..Default::default()
            }
        ))

    }).await
}


// subscribe rss feed
#[post("/unsubscribe", format="application/x-www-form-urlencoded", data="<crate_id>")]
pub async fn unsubscribe(token: Token, crate_id: String, conn: DbConn) -> Result<Value, RssError> {

    let username = token.0;

    let original = RawStr::new(&crate_id).url_decode().unwrap();
    let crate_id: Vec<&str> = original.split('=').collect();
    let crate_id = crate_id[1].to_string();

    conn.run(move |con| {
        Subscribe::delete(username.clone(), crate_id.clone(), &con)?;
        Task::delete(username.clone(), crate_id.clone(), &con)?;
        Message::delete(username, crate_id, con)?;

        Ok(json!(
            ResMsg {
                status: SuccessStatus::UNSUBSCRIBE,
                msg: String::from("success"),
                ..Default::default()
            }
        ))
    }).await

}