use diesel::{SqliteConnection, QueryDsl ,RunQueryDsl, prelude::*};
use crate::error::RssError;
use crate::types::schema::message;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "message"]
pub struct Message {
    pub id: Option<i32>,
    pub message_id: String,
    pub crate_id: String,
    pub receiver: String,
    pub check_status: bool,
    pub notify_status: bool,
}
impl Message {
    pub fn insert(&self, con: &SqliteConnection) -> Result<(), RssError> {
        
        diesel::insert_into(message::table).values(self).execute(con)?;
    
        Ok(())
    }
    pub fn delete(username: String, crate_id: String, con: &SqliteConnection) -> Result<(), RssError> {
        diesel::delete(message::table.filter(message::receiver.eq(username).and(message::crate_id.eq(crate_id)))).execute(con)?;
        Ok(())
    }

    pub fn get_records_by_receiver(receiver: String, con: &SqliteConnection) -> Result<Vec<Self>, RssError> {
        
        let query = message::table
                        .filter(message::receiver.eq(receiver))
                        .load::<Self>(con)?;
        Ok(query)
    }
    pub fn get_records_by_receiver_notify(receiver: String, con: &SqliteConnection) -> Result<Vec<Self>, RssError> {
        
        let query = message::table
                        .filter(message::receiver.eq(receiver).and(message::notify_status.eq(false)))
                        .load::<Self>(con)?;
        Ok(query)
    }
    
    pub fn set_check_status_by_messageid(
        message_id: String,
        status: bool,
        con: &SqliteConnection
    ) -> Result<(), RssError> {

        diesel::update(message::table.filter(message::message_id.eq(message_id)))
            .set(message::check_status.eq(status))
            .execute(con)?;
        Ok(())
    }

    pub fn set_notify_status_by_messageid(
        message_id: String,
        status: bool,
        con: &SqliteConnection
    ) -> Result<(), RssError> {

        diesel::update(message::table.filter(message::message_id.eq(message_id)))
            .set(message::notify_status.eq(status))
            .execute(con)?;
        Ok(())
    }
}
impl Default for Message {
    fn default() -> Self {
        Self {
            id: None,
            message_id: String::new(),
            crate_id: String::new(),
            receiver: String::new(),
            check_status: false,
            notify_status: false,
        }
    }
}



