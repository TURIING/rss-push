use crate::{
    types::{
        ResMsg,
        SuccessStatus,
        auth::Token, database::CrateInfo, ResCrateInfo,
    },
    DbConn,
    error::RssError,
    database,
    rss::RssInfo,
};
use rocket::{
    serde::json::{serde_json::json, Value, self},
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
    let crate_info = CrateInfo{ url, title, description };
    
    conn.run(move |con| {
        // check exist.
        database::crates::exist_or_insert(con, url_uuid.clone(), crate_info)?;

        // Join the task queue.
        database::task::insert(con, url_uuid.clone(), username.clone())?;

        database::subscribe::insert(con, username, url_uuid)?;

        Ok(json!(ResMsg{ status: SuccessStatus::SUBSCRIBE, msg: String::from("success"), ..Default::default()}))

    }).await
}

// Returns all subscriptions for the specified user
#[get("/subscribed")]
pub async fn subscribed(token: Token, conn: DbConn) -> Result<Value, RssError> {
    
    let username = token.0;
    conn.run(|con| {
        if let Some(records) = database::subscribe::get_records_by_username(con, username)? {
            let mut res_crates_info: Vec<ResCrateInfo> = Vec::new();
            for crate_id in records {
                let crate_info = database::crates::get_info_by_id(con, crate_id.clone())?;
                let crate_info: CrateInfo = json::from_str(&crate_info).unwrap();
                let res_crate_info: ResCrateInfo = ResCrateInfo { crate_id, crate_info};
                res_crates_info.push(res_crate_info);
            }
            Ok(json!(
                ResMsg{ status: SuccessStatus::SUBSCRIBED_SOME, crates_info: res_crates_info, ..Default::default()}
            ))
        } else {
            Ok(json!(ResMsg{ status: SuccessStatus::SUBSCRIBED_NONE, ..Default::default()}))
        }
    }).await

}


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
