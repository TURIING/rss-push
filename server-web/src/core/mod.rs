use std::str::FromStr;

use diesel::prelude::*;

use crate::constant::DATABASEURL;
use crate::error::RssError;
use crate::database;
use crate::rss::Rss;
use crate::types::database::CrateInfo;
use uuid::Uuid;
use rocket::serde::json;
use std::sync::Arc;
use rocket::tokio::sync::Mutex;

pub async fn check_update() -> Result<(), RssError> {
    let con = SqliteConnection::establish(DATABASEURL).unwrap();

    let query = database::task::get_records_by_id(&con, 0, 100)?;
    if let Some(query) = query {
        for record in query {
            if record.task_type == "rss" {
                let crate_info = database::crates::get_info_by_id(&con, record.crate_id.clone())?;
                let crate_info: CrateInfo = json::from_str(&crate_info).unwrap();
                let CrateInfo { url, ..} = crate_info;
                let update = Rss::get_update(url, None).await?;
                if let Some(update) = update {
                    for item in update {
                        database::message_content::insert(item.uuid.clone(), Some(item.pub_date), item.description, &con)?;
                        database::message::insert(item.uuid, record.crate_id.clone(), record.username.clone(), &con)?;
                    }
                } else {
                    todo!()
                }
            }
        }
    } else {
        todo!()
    }

    // if let Some(query) = database::task::get_records_by_id(&con, 0, 100)? {
        
    //     for record in query {
    //         if record.task_type == "rss" {
    //             let crate_info = database::crates::get_info_by_id(&con, record.crate_id)?;
    //             let crate_info: CrateInfo = json::from_str(&crate_info).unwrap();
    //             let CrateInfo { url, ..} = crate_info;
    //             for item in Rss::get_update(url, None).await?.unwrap() {
    //                 println!("{:?}", item.title);
    //             }
                
                
    //         } else {
    //             todo!()
    //         }
    //     }
    // } else {
    //     println!("none");
    // }

    Ok(())
}