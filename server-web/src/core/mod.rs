use std::str::FromStr;
use diesel::prelude::*;
use crate::constant::DATABASEURL;
use crate::error::RssError;
use crate::database::{task::Task, crates::Crates, message_content::MessageContent, message::Message};
use crate::rss::Rss;
use crate::types::{ CrateInfo, ResMessagesInfo };
use uuid::Uuid;
use rocket::serde;
use std::time::{Instant, Duration};
use rocket::tokio::{self, sync::Mutex, net::{TcpListener, TcpStream}};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_tungstenite::{tungstenite, accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};


pub async fn check_update() -> Result<(), RssError> {
    let con = SqliteConnection::establish(DATABASEURL).unwrap();
    let mut id_loop: usize = 0;
    loop {
        
        let time = Instant::now();

        let query = Task::get_records_by_id(&con, id_loop.clone() as i32, 1)?;

        if let Some(query) = query {

            for record in query {
                
                let crate_info = Crates::get_info_by_id(&con, record.crate_id.clone())?;
                let crate_info: CrateInfo = serde::json::from_str(&crate_info).unwrap();
                let CrateInfo { url, ..} = crate_info;
                id_loop = record.id.unwrap() as usize;
                
                let last_msg_uuid = record.params.map(|s| Uuid::from_str(&s).unwrap());
                let update = Rss::get_update(url, last_msg_uuid).await?;
                if let Some(update) = update {
                    let mut index = 1;
                    for item in update {
                        if index == 1 {
                            Task::update_params_by_id(record.id.unwrap(), item.uuid.clone(), &con)?;
                            index = 0;
                        }
                        MessageContent { 
                            id: None, 
                            message_id: item.uuid.clone(), 
                            send_time: item.pub_date, 
                            content: item.content, 
                            title: item.title,
                            description: item.description
                        }.insert(&con)?;

                        Message {
                            message_id: item.uuid, 
                            crate_id: record.crate_id.clone(), 
                            receiver: record.username.clone(), 
                            ..Default::default()
                        }.insert(&con)?;                        
                    }
                }
            }
        } else {
            id_loop = 0;
        }
        if time.elapsed().as_secs() < 3 {                
            info!("Idle for 1 seconds");
            rocket::tokio::time::sleep(Duration::new(1, 0)).await;
        }
    }          
}

pub async fn notify_launch() -> anyhow::Result<()> {
    let mut stream_list: HashMap<String, WebSocketStream<TcpStream>> = HashMap::new();
    let stream_list = Arc::new(Mutex::new(stream_list));
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    
    
    tokio::spawn(send_update(Arc::clone(&stream_list)));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, Arc::clone(&stream_list)));
    }
        
    Ok(())
}
pub async fn send_update(stream_list: Arc<Mutex<HashMap<String, WebSocketStream<TcpStream>>>>) -> anyhow::Result<()> {
    let con = SqliteConnection::establish(DATABASEURL).unwrap();
    loop {
        let mut messages_info: Vec<ResMessagesInfo> = Vec::new();
        // 遍历在线的用户
        for (username,stream) in &mut *(stream_list.lock().await) {
            let message_query = Message::get_records_by_receiver_notify(username.to_owned(), &con)?;            
            for message in message_query {
                // 加入推送队列
                let mc = MessageContent::get_records_by_messageid(message.message_id.clone(), &con)?;
                messages_info.push(
                    ResMessagesInfo { 
                        title: mc.title, 
                        description: mc.description,
                        content: mc.content, 
                        message_id: message.message_id.clone(), 
                        crate_id: message.crate_id, 
                        check_status: message.check_status, 
                        send_time: mc.send_time
                    }
                );
                // 标志为已推送
                Message::set_notify_status_by_messageid(message.message_id.clone(), true, &con)?;
            }
            // 推送信息队列不为空，则推送
            if !messages_info.is_empty() {
                let info = serde::json::json!(messages_info);
            
                stream.send(tungstenite::Message::Text(info.to_string())).await?;
                info!("Push messages to {}", username);
            }
            
        }       
        rocket::tokio::time::sleep(Duration::new(1, 0)).await;            
        
    }
    Ok(())
}

pub async fn accept_connection(stream: TcpStream, stream_list: Arc<Mutex<HashMap<String, WebSocketStream<TcpStream>>>>) -> anyhow::Result<()>{
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    if let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let mut stream_list = stream_list.lock().await;
            stream_list.insert(msg.clone().into_text()?, ws_stream);
            info!("Establish a Websocket connection with {}", msg.into_text()?);
        }
    }
    
    
    Ok(())
}

