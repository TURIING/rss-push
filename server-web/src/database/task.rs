use diesel::{SqliteConnection, QueryDsl, RunQueryDsl, prelude::*};

use crate::types::schema::task;
use crate::error::RssError;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "task"]
pub struct Task {
    pub id: Option<i32>,
    pub crate_id: String,
    // 0: rss, 1: sdk
    pub task_type: i32,
    pub username: String,
    pub params: Option<String>
}

impl Task {
    pub fn insert(&self, con: &SqliteConnection) -> Result<(), RssError> {
        
        diesel::insert_into(task::table).values(self).execute(con)?;
        Ok(())
    }

    pub fn get_records_by_id(
        con: &SqliteConnection,
        id: i32, 
        limit: i64
    ) -> Result<Option<Vec<Self>>, RssError> {
        
        let tasks = task::table.filter(task::id.gt(id)).limit(limit).load::<Self>(con)?;
        if tasks.len() != 0 {
            Ok(Some(tasks))
        } else {
            Ok(None)
        }
    }
    pub fn delete(username: String, crate_id: String, con: &SqliteConnection) -> Result<(), RssError> {
        diesel::delete(task::table.filter(task::username.eq(username).and(task::crate_id.eq(crate_id)))).execute(con)?;
        Ok(())
    }

    pub fn update_params_by_id(id: i32, params: String, con: &SqliteConnection) -> Result<(), RssError> {
        diesel::update(task::table.filter(task::id.eq(id)))
            .set(task::params.eq(params))
            .execute(con)?;
        Ok(())
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: None,
            crate_id: String::new(),
            task_type: 0,
            username: String::new(),
            params: None,
        }
    }
}




