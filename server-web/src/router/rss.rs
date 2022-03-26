use crate::{
    types::{
        database::{ crates, task},
        rss::SubscribeInfo,
        ResMsg,
        task::{ TaskQuery, CratesQuery },
        SuccessStatus,
        auth::Token,
    },
    utility::get_username_by_session,
    DbConn,
    error::RssError,
    database,
};
use diesel::{QueryDsl, RunQueryDsl, prelude::*, result::Error::NotFound};
use rocket::{
    form::{Form, Strict},
    serde::json::{serde_json::json, Value, Json},
    http::RawStr,
};
use rss::Channel;
use uuid::Uuid;


// return rss info
#[post("/info", format="application/x-www-form-urlencoded", data="<url>")]
pub async fn info(_token: Token, url: String) -> Result<Value, RssError> {
    let url_original = RawStr::new(&url).url_decode().unwrap();
    let url: Vec<&str> = url_original.split('=').collect();

    let content = reqwest::get(url[1])
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    
    Ok(json!(ResMsg{ status: SuccessStatus::RSSINFO, title: channel.title, description: channel.description, ..Default::default()}))
}

// subscribe rss feed
#[post("/subscribe", format="application/x-www-form-urlencoded", data="<url>")]
pub async fn subscribe(token: Token, url: String, conn: DbConn) -> Result<Value, RssError> {

    let url_original = RawStr::new(&url).url_decode().unwrap();
    let url: Vec<&str> = url_original.split('=').collect();
    let url_uuid = Uuid::new_v3(&Uuid::NAMESPACE_URL, url[1].as_bytes()).to_string();
    let username = token.0;
    
    conn.run(move |con| {

        // check exist.
        database::crates::exist_or_insert(con, url_uuid.clone())?;

        // Join the task queue.
        database::task::insert(con, url_uuid, username)?;

        Ok(json!(ResMsg{ status: SuccessStatus::SUBSCRIBE, msg: String::from("success"), ..Default::default()}))

    }).await
}

// #[post("/item", data = "<item_id>")]
// pub fn item(item_id: Form<Strict<String>>) -> Result<Value, Error> {

//     let item_url = 
    
// }


#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use rss::Channel;
    use rocket::tokio;
    #[test]
    fn it_works() {
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
    }
    #[rocket::tokio::test]
    async fn rss() {
        let url = "http://feed.williamlong.info/";
        let content = reqwest::get(url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let channel = Channel::read_from(&content[..]).unwrap();
        for i in channel.items.iter() {
            if let Some(title) = &i.title {
                println!("{}", title);
            }
        }
    }
}
