

use crate::{
    types::{
        database::{ crates, task, login_state},
        rss::{ RssInfo, SubscribeInfo, CratesQuery, TaskQuery },
        ResMsg
    },
    utility::get_username_by_session,
    DbConn,
    error::Error
};
use diesel::{QueryDsl, RunQueryDsl};
use rocket::{
    form::{Form, Strict},
    serde::json::{serde_json::json, Value, Json},
};
use rss::Channel;
use uuid::Uuid;



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
pub async fn subscribe(info: Json<SubscribeInfo>, conn: DbConn) -> Result<Value, Error> {
    let { url, session_data } = info.into_inner();
    let url_uuid = Uuid::new_v3(&Uuid::NAMESPACE_URL, url).to_string();
    
    conn.run(move |con| {
        // Check whether this record exists in the Crates table
        let crate_query = crates::table
            .filter(crates::crates_id.eq(url_uuid.clone()))
            .execute(con);

        if let Err(e) = crate_query {
            // Create a crate record
            let crate_info = CratesQuery {
                crates_id: url_uuid.clone(),
                crates_type: String::from("rss"),
                info: String::new()
            };
            diesel::insert_into(crates::table).values(crate_info).execute(con).unwrap();
        }
        // Join the task queue
        let username = get_username_by_session(session_data, con)
            .map_err(|e| return e);

        let task_info = TaskQuery {
            crates_id: String::from(url_uuid),
            task_type: String::from("rss"),
            username,
            params: String::new()
        };
        diesel::insert_into(task::table).values(task_info).execute(con)?;
        Ok(json!(ResMsg{ status: 202, msg: String::from("subscribe success"), ..Default::default()}))
    }).await;

    
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
