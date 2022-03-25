

use crate::{
    types::{
        database::{ crates, task},
        rss::{ RssInfo, SubscribeInfo },
        ResMsg,
        task::{ TaskQuery, CratesQuery },
        SuccessStatus,
        auth::Token,
    },
    utility::get_username_by_session,
    DbConn,
    error::RssError
};
use diesel::{QueryDsl, RunQueryDsl, prelude::*, result::Error::NotFound};
use rocket::{
    form::{Form, Strict},
    serde::json::{serde_json::json, Value, Json},
};
use rss::Channel;
use uuid::Uuid;


// return rss info
#[post("/info", data = "<url>")]
pub async fn info(token: Token, url: Form<Strict<String>>) -> Result<Value, RssError> {
    let content = reqwest::get(url.as_str())
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    
    Ok(json!(ResMsg{ status: SuccessStatus::RSSINFO, title: channel.title, description: channel.description, ..Default::default()}))
}

// subscribe rss feed
#[post("/subscribe", data = "<info>")]
pub async fn subscribe(info: Json<SubscribeInfo>, conn: DbConn) -> Result<Value, RssError> {

    let SubscribeInfo{ url, session_data } = info.into_inner();
    let url_uuid = Uuid::new_v3(&Uuid::NAMESPACE_URL, url.as_bytes()).to_string();
    
    conn.run(move |con| {
        // verify session_data
        let username = get_username_by_session(session_data, con)?;

        // Check whether this record exists in the Crates table
        crates::table
            .filter(crates::crates_id.eq(url_uuid.clone()))
            .select(crates::crates_id)
            .first::<String>(con)
            .or_else(|e| {
                match e {
                    NotFound => {
                        let crate_info = CratesQuery {
                            crates_id: url_uuid.clone(),
                            crates_type: String::from("rss"),
                            info: String::new()
                        };
                        diesel::insert_into(crates::table).values(crate_info).execute(con)?;
                        Ok(String::new())
                    },
                    _ => return Err(e),
                }
            })?;

        // Join the task queue
        let task_info = TaskQuery {
            crates_id: String::from(url_uuid),
            task_type: String::from("rss"),
            username,
            params: String::new()
        };
        diesel::insert_into(task::table).values(task_info).execute(con)?;

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
    #[test]
    fn it_works() {
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
    }
}
