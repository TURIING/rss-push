use diesel::{SqliteConnection, QueryDsl, RunQueryDsl, prelude::*};

use crate::types::{ database::TaskQuery, schema::task };
use crate::error::RssError;


pub fn get_records_by_id(
    con: &SqliteConnection, 
    id: i32, 
    limit: i64
) -> Result<Option<Vec<TaskQuery>>, RssError> {
    
    let tasks = task::table.filter(task::id.gt(id)).limit(limit).load::<TaskQuery>(con)?;
    if tasks.len() != 0 {
        Ok(Some(tasks))
    } else {
        Ok(None)
    }
}

pub fn insert(
    con: &SqliteConnection,
    url_uuid: String,
    username: String
) -> Result<(), RssError> {
    let task_info = TaskQuery {
        id: None,
        crate_id: String::from(url_uuid),
        task_type: String::from("rss"),
        username,
        params: String::new()
    };
    diesel::insert_into(task::table).values(task_info).execute(con)?;
    Ok(())
}