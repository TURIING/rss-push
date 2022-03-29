use std::str::FromStr;

use diesel::prelude::*;

use crate::constant::DATABASEURL;
use crate::error::RssError;
use crate::database;
use crate::rss::Rss;
use crate::types::database::CrateInfo;
use uuid::Uuid;
use rocket::serde::json;

pub async fn check_update() -> Result<(), RssError> {
    let con = SqliteConnection::establish(DATABASEURL)
        .expect("The database URL could not be found");

    if let Some(query) = database::task::get_records_by_id(&con, 0, 100)? {
        
        for record in query {
            if record.task_type == "rss" {
                let crate_info = database::crates::get_info_by_id(&con, record.crate_id)?;
                let crate_info: CrateInfo = json::from_str(&crate_info).unwrap();
                let CrateInfo { url, ..} = crate_info;
                for item in Rss::get_update(url, None).await?.unwrap() {
                    println!("{:?}", item.title);
                }
                
                
            } else {
                todo!()
            }
        }
    } else {
        println!("none");
    }

    Ok(())
}