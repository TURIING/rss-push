

use crate::{
    types::{
        database::{ crates, task},
        rss::{ RssInfo, SubscribeInfo, CratesQuery, TaskQuery },
        ResMsg
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
//afb79651-4dcd-4400-903c-5b9fa1efd019


// return rss info
#[post("/info", data = "<url>")]
pub async fn info(url: Form<Strict<String>>) -> Value {
    let content = reqwest::get(url.as_str())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = Channel::read_from(&content[..]).unwrap();
    json!(RssInfo {
        title: channel.title,
        description: channel.description
    })
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

        Ok(json!(ResMsg{ status: 202, msg: String::from("subscribe success"), ..Default::default()}))

    }).await

    
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;
    #[test]
    fn it_works() {
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
        println!("{:?}", Uuid::new_v3(&Uuid::NAMESPACE_URL, "http://baidu.com".as_bytes()));
    }
}
