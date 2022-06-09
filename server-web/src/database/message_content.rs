use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
use crate::types::schema::message_content;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "message_content"]
pub struct MessageContent {
    pub id: Option<i32>,
    pub message_id: String,
    pub send_time: String,
    pub content: String,
    pub title: String,
    pub description: String,
}
impl MessageContent {
    pub fn insert(&self, con: &SqliteConnection) -> Result<(), RssError> {
        
        diesel::insert_into(message_content::table).values(self).execute(con)?;
    
        Ok(())
    }

    pub fn get_records_by_messageid(message_id: String, con: &SqliteConnection) -> Result<Self, RssError> {
        
        let query = message_content::table
                        .filter(message_content::message_id.eq(message_id))
                        .first::<Self>(con)?;
        Ok(query)
    }
}




