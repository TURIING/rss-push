use crate::{
    types::{ResMsg, SuccessStatus, auth::Token, CrateInfo, ResCrateInfo, ResMessagesInfo},
    DbConn, error::RssError,
    database::{subscribe::Subscribe, crates::Crates, message::Message, message_content::MessageContent},
};
use rocket::{
    serde::json::{serde_json::json, Value, self},
    http::RawStr
};

// Returns all subscriptions for the specified user
#[get("/subscribed")]
pub async fn subscribed(token: Token, conn: DbConn) -> Result<Value, RssError> {
    
    let username = token.0;
    conn.run(|con| {
        if let Some(records) = Subscribe::get_records_by_username(con, username)? {
            let mut res_crates_info: Vec<ResCrateInfo> = Vec::new();
            for crate_id in records {
                let crate_info = Crates::get_info_by_id(con, crate_id.clone())?;
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
#[get("/refresh")]
pub async fn refresh_message(token: Token, conn: DbConn) -> Result<Value, RssError> {
    let username = token.0;
    conn.run(|con| {
        let message_query = Message::get_records_by_receiver(username, con)?;
        let mut messages_info: Vec<ResMessagesInfo> = Vec::new();
        for message in message_query {
            // 设置推送标志
            Message::set_notify_status_by_messageid(message.message_id.clone(), true, con)?;
            // 加入返回信息队列
            let mc = MessageContent::get_records_by_messageid(message.message_id.clone(), con)?;
            messages_info.push(
                ResMessagesInfo { 
                    title: mc.title, 
                    description: mc.description, 
                    content: mc.content, 
                    message_id: message.message_id, 
                    crate_id: message.crate_id, 
                    check_status: message.check_status, 
                    send_time: mc.send_time
                }
            )
        }
        if messages_info.len() == 0 {
            Ok(json!(
                ResMsg { 
                    status: SuccessStatus::REFRESHMESSAGE_NONE,
                    ..Default::default() }
            ))
        } else {
            Ok(json!(
                ResMsg { 
                    status: SuccessStatus::REFRESHMESSAGE_SOME,
                    messages_info, 
                    ..Default::default() }
            ))
        }
        
    }).await
}

#[post("/read", format="application/x-www-form-urlencoded", data="<message_id>")]
pub async fn read_message(message_id: String, conn: DbConn) -> Result<Value, RssError> {
    let original = RawStr::new(&message_id).url_decode().unwrap();
    let message_id: Vec<&str> = original.split('=').collect();
    let message_id = message_id[1].to_string();

    conn.run(|con| {
        Message::set_check_status_by_messageid(message_id, true, con)?;
        Ok(json!(ResMsg {status: SuccessStatus::READMESSAGE, ..Default::default()}))
    }).await
}

#[post("/unread", format="application/x-www-form-urlencoded", data="<message_id>")]
pub async fn unread_message(message_id: String, conn: DbConn) -> Result<Value, RssError> {
    let original = RawStr::new(&message_id).url_decode().unwrap();
    let message_id: Vec<&str> = original.split('=').collect();
    let message_id = message_id[1].to_string();

    conn.run(|con| {
        Message::set_check_status_by_messageid(message_id, false, con)?;
        Ok(json!(ResMsg {status: SuccessStatus::UNREADMESSAGE, ..Default::default()}))
    }).await
}